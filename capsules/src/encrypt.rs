//! Encryption Capsule
//!
//!
//! Provides a simple driver for userspace applications to encrypt and decrypt messages


use core::cell::Cell;
use kernel::{AppId, AppSlice, Container, Callback, Driver, ReturnCode, Shared};
use kernel::common::take_cell::{MapCell, TakeCell};
use kernel::hil::aes::{AESDriver, Client};
use kernel::process::Error;

pub static mut BUF: [u8; 64] = [0; 64];

pub struct App {
    callback: Option<Callback>,
    key_buf: Option<AppSlice<Shared, u8>>,
    pt_buf: Option<AppSlice<Shared, u8>>,
    ct_buf: Option<AppSlice<Shared, u8>>,
}


impl Default for App {
    fn default() -> App {
        App {
            callback: None,
            key_buf: None,
            pt_buf: None,
            ct_buf: None,
        }
    }
}


pub struct Encrypt<'a, E: AESDriver + 'a> {
    enc: &'a E,
    apps: Container<App>,
    kernel_key: TakeCell<'static, [u8]>,
    kernel_pt: TakeCell<'static, [u8]>,
    kernel_ct: TakeCell<'static, [u8]>,
}

impl<'a, E: AESDriver + 'a> Encrypt<'a, E> {
    pub fn new(enc: &'a E,
               container: Container<App>,
               kbuf: &'static mut [u8],
               pbuf: &'static mut [u8],
               cbuf: &'static mut [u8])
               -> Encrypt<'a, E> {
        Encrypt {
            enc: enc,
            apps: container,
            kernel_key: TakeCell::new(kbuf),
            kernel_pt: TakeCell::new(pbuf),
            kernel_ct: TakeCell::new(cbuf),
        }
    }
}

impl<'a, E: AESDriver + 'a> Client for Encrypt<'a, E> {
    fn encrypt_done(&self, ct: &'static mut [u8]) -> ReturnCode {
        
        // ISSUE HOW DO WEED ADD APPDATA TO POINT AT CT BUF
        // TODO
        for cntr in self.apps.iter() {
            cntr.enter(move |app, _| { 
                    app.callback.map(|mut cb| { 
                        cb.schedule( 0, 4, 3); 
                    }); 
            });
        }
        ReturnCode::SUCCESS
    }

    fn decrypt_done(&self, pt: &'static mut [u8]) -> ReturnCode {
        ReturnCode::SUCCESS
    }

    #[inline(never)]
    #[no_mangle]
    fn set_key_done(&self, key: &'static mut [u8]) -> ReturnCode {
        for cntr in self.apps.iter() {
            cntr.enter(|app, _| { app.callback.map(|mut cb| { cb.schedule(0, 0, 0); }); });
        }
        ReturnCode::SUCCESS
    }
}


impl<'a, E: AESDriver> Driver for Encrypt<'a, E> {
    fn allow(&self, appid: AppId, allow_num: usize, slice: AppSlice<Shared, u8>) -> ReturnCode {
        match allow_num {
            0 => {
                self.apps
                    .enter(appid, |app, _| {
                        app.key_buf = Some(slice);
                        ReturnCode::SUCCESS
                    })
                    .unwrap_or_else(|err| match err {
                        Error::OutOfMemory => ReturnCode::ENOMEM,
                        Error::AddressOutOfBounds => ReturnCode::EINVAL,
                        Error::NoSuchApp => ReturnCode::EINVAL,
                    })
            }
            1 => {
                self.apps
                    .enter(appid, |app, _| {
                        app.pt_buf = Some(slice);
                        ReturnCode::SUCCESS
                    })
                    .unwrap_or_else(|err| match err {
                        Error::OutOfMemory => ReturnCode::ENOMEM,
                        Error::AddressOutOfBounds => ReturnCode::EINVAL,
                        Error::NoSuchApp => ReturnCode::EINVAL,
                    })
            }
            2 => panic!("DECRYPTION FOR AES ECB IS NOT PRESENT"),

            _ => ReturnCode::ENOSUPPORT,
        }
    }

    fn subscribe(&self, subscribe_num: usize, callback: Callback) -> ReturnCode {
        match subscribe_num {
            0 => {
                self.apps
                    .enter(callback.app_id(), |app, _| {
                        app.callback = Some(callback);
                        ReturnCode::SUCCESS
                    })
                    .unwrap_or_else(|err| match err {
                        Error::OutOfMemory => ReturnCode::ENOMEM,
                        Error::AddressOutOfBounds => ReturnCode::EINVAL,
                        Error::NoSuchApp => ReturnCode::EINVAL,
                    })
            }
            _ => ReturnCode::ENOSUPPORT,
        }
    }
    fn command(&self, command_num: usize, data: usize, appid: AppId) -> ReturnCode {
        match command_num {
            0 => {
                for cntr in self.apps.iter() {
                    cntr.enter(|app, _| {
                        app.key_buf.as_mut().map(|slice| {

                            self.kernel_key.take().map(|buf| {
                                for (i, c) in slice.as_ref()[0..16]
                                    .iter()
                                    .enumerate() {
                                    if buf.len() < i {
                                        break;
                                    }
                                    buf[i] = *c;
                                }
                                self.enc.set_key(buf);
                            });

                        });
                    });
                }
                ReturnCode::SUCCESS
            }
            1 => {
                for cntr in self.apps.iter() {
                    cntr.enter(|app, _| {
                        app.pt_buf.as_mut().map(|slice| {

                            self.kernel_pt.take().map(|buf| {
                                for (i, c) in slice.as_ref()[0..16]
                                    .iter()
                                    .enumerate() {
                                    if buf.len() < i {
                                        break;
                                    }
                                    buf[i] = *c;
                                }
                                self.enc.encrypt(buf);
                            });

                        });
                    });
                }
                ReturnCode::SUCCESS
            }
            2 => panic!("DECRYPTION FOR AES ECB IS NOT PRESENT"),
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}
