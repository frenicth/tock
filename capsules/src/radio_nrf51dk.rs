use core::cell::Cell;
use kernel::{AppId, Driver, Callback, AppSlice, Shared, Container};
use kernel::common::take_cell::{MapCell, TakeCell};
use kernel::hil::radio_nrf51dk::{RadioDummy, Client};
use kernel::process::Error;
use kernel::returncode::ReturnCode;
static mut BUF: [u8; 16] = [0; 16];

pub struct App {
    //tx_callback: Option<Callback>,
    rx_callback: Option<Callback>,
    app_read: Option<AppSlice<Shared, u8>>,
    app_write: Option<AppSlice<Shared, u8>>,
}

impl Default for App {
    fn default() -> App {
        App {
            //tx_callback : None,
            rx_callback: None,
            app_read: None,
            app_write: None,
        }
    }
}

pub struct Radio<'a, R: RadioDummy + 'a> {
    radio: &'a R,
    busy: Cell<bool>,
    app: Container<App>,
    kernel_tx: TakeCell<'static, [u8]>,
}
// 'a = lifetime
// R - type Radio
impl<'a, R: RadioDummy + 'a> Radio<'a, R> {
    pub fn new(radio: &'a R, container: Container<App>) -> Radio<'a, R> {
        Radio {
            radio: radio,
            busy: Cell::new(false),
            app: container,
            kernel_tx: TakeCell::empty(),
        }
    }

    pub fn capsule_init(&self) {
        // call chips::radio::init()
        self.radio.init()
    }
    pub fn config_buffer(&self) {
        unsafe {
            self.kernel_tx.replace(&mut BUF);
        }
    }
}

impl<'a, R: RadioDummy + 'a> Client for Radio<'a, R> {
    #[inline(never)]
    #[no_mangle]
    fn receive_done(&self, rx_data: &'static mut [u8], rx_len: u8) -> ReturnCode {
        for cntr in self.app.iter() {
            cntr.enter(|app, _| { app.rx_callback.map(|mut cb| { cb.schedule(0, 0, 0); }); });
        }
        ReturnCode::SUCCESS
    }
}
// Implementation of the Driver Trait/Interface
impl<'a, R: RadioDummy + 'a> Driver for Radio<'a, R> {
    //  0 -  configure  channel/frequency,       data: 0 ... x channel/freq
    //  1 -  rx
    //  2 -  tx
    //  3 -  configure crc settings
    //  4 -  set address
    fn command(&self, command_num: usize, data: usize, _: AppId) -> ReturnCode {
        // panic!("command \n");
        match command_num {
            0 => {
                self.radio.receive();
                ReturnCode::SUCCESS
            }
            1 => {
                for cntr in self.app.iter() {
                    cntr.enter(|app, _| {
                        app.app_write.as_mut().map(|slice| {

                            self.kernel_tx.take().map(|buf| {
                                for (i, c) in slice.as_ref()[0..16]
                                    .iter()
                                    .enumerate() {
                                    if buf.len() < i {
                                        break;
                                    }
                                    buf[i] = *c;
                                }
                                self.radio.transmit(0, buf, 16);
                            });

                        });
                    });
                }
                self.config_buffer();
                ReturnCode::SUCCESS
            }
            _ => ReturnCode::EALREADY,
        }
    }

    fn subscribe(&self, subscribe_num: usize, callback: Callback) -> ReturnCode {
        match subscribe_num {
            0 => {
                // panic!("subscribe_rx");
                self.app
                    .enter(callback.app_id(), |app_tmp, _| {
                        app_tmp.rx_callback = Some(callback);
                        ReturnCode::SUCCESS
                    })
                    .unwrap_or_else(|err| match err {
                        _ => ReturnCode::ENOSUPPORT,
                    })
            }
            _ => ReturnCode::ENOSUPPORT,
        }
    }

    fn allow(&self, appid: AppId, allow_num: usize, slice: AppSlice<Shared, u8>) -> ReturnCode {
        // panic!("allow error\n");
        match allow_num {
            0 => {
                self.app
                    .enter(appid, |app, _| {
                        app.app_read = Some(slice);
                        ReturnCode::SUCCESS
                    })
                    .unwrap_or_else(|err| match err {
                        Error::OutOfMemory => ReturnCode::ENOMEM,
                        Error::AddressOutOfBounds => ReturnCode::EINVAL,
                        Error::NoSuchApp => ReturnCode::EINVAL,
                    })
            }
            1 => {
                self.app
                    .enter(appid, |app, _| {
                        app.app_write = Some(slice);
                        ReturnCode::SUCCESS
                    })
                    .unwrap_or_else(|err| match err {
                        Error::OutOfMemory => ReturnCode::ENOMEM,
                        Error::AddressOutOfBounds => ReturnCode::EINVAL,
                        Error::NoSuchApp => ReturnCode::EINVAL,
                    })
            }
            _ => panic!("WZZUP"),

        }
    }
}
