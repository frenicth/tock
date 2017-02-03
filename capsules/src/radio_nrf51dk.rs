use core::cell::Cell;
use kernel::{AppId, Driver, Callback, AppSlice, Shared};
use kernel::common::take_cell::{MapCell, TakeCell};
use kernel::hil::radio_nrf51dk::RadioDummy;
use kernel::returncode::ReturnCode;

struct App {
    tx_callback: Option<Callback>,
    rx_callback: Option<Callback>,
    app_read: Option<AppSlice<Shared, u8>>,
    app_write: Option<AppSlice<Shared, u8>>,
}

// FIX THIS ATTRIBUTES LATER
pub struct Radio<'a, R: RadioDummy + 'a> {
    radio: &'a R,
    busy: Cell<bool>,
    app: MapCell<App>,
    kernel_tx: TakeCell<'static, [u8]>,
}

// 'a = lifetime
// R - type Radio
impl<'a, R: RadioDummy + 'a> Radio<'a, R> {
    pub fn new(radio: &'a R) -> Radio<'a, R> {
        Radio {
            radio: radio,
            busy: Cell::new(false),
            app: MapCell::empty(),
            kernel_tx: TakeCell::empty(),
        }
    }

    pub fn capsule_init(&self) {
        // call chips::radio::init()
        self.radio.init()
    }

    // TODO ADD MORE FUNCTIONS
}

// Implementation of the Driver Trait/Interface
impl<'a, R: RadioDummy + 'a> Driver for Radio<'a, R> {
    #[inline(never)]
    #[no_mangle]
    fn command(&self, command_num: usize, data: usize, _: AppId) -> ReturnCode {
        // self.radio.init();
        // call tx / rx HEJSAN
        //self.radio.init();
        self.radio.send();
        self.radio.receive();
        ReturnCode::SUCCESS
    }

    fn subscribe(&self, subscribe_num: usize, callback: Callback) -> ReturnCode {
        // panic!("");
        match subscribe_num {
            // subscribe to all pin interrupts
            // (no affect or reliance on individual pins being configured as interrupts)
            0 => {
                // panic!("");
                //self.callback.set(Some(callback));
                // r0: usize, r1: usize, r2: usize
                // self.callback.get().unwrap().schedule(0, 0, 0);
                ReturnCode::SUCCESS

            }

            // default
            _ => ReturnCode::SUCCESS,
        }
    }

    fn allow(&self, _appid: AppId, allow_num: usize, slice: AppSlice<Shared, u8>) -> ReturnCode {
        // panic!("appSlice {:?}", slice.len());
        let appc = match self.app.take() {
            None => {
                App {
                    tx_callback: None,
                    rx_callback: None,
                    app_read: Some(slice),
                    app_write: None,
                }
            }

            Some(mut i) => {
                i.app_read = Some(slice);
                i
            }
        };
        self.app.replace(appc);
        let mut  kbuf: [u8; 16] = [0; 16];
        self.app.map(|app| {

            // se2lf.kernel_tx.map(|kbuf| {
            app.app_read.as_mut().map(|src| {
                for (i, c) in src.as_ref()[0..16].iter().enumerate() {
                    // panic!("looping i {:?} c {:?}", i, *c);
                    kbuf[i] = *c;
                }

            });
        });

        // self.kernel_tx.replace(&kbuf);
        // self.radio.transmit(0, kbuf, 16);
        // panic!("kbuf {:?}", kbuf);
        ReturnCode::SUCCESS

            // panic!("MapCell {:?}", self.app.take());
    }
}
