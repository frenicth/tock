//! Encryption Capsule
//!
//!
//! Provides a simple driver for userspace applications to encrypt and decrypt messages


use core::cell::Cell;
use kernel::{AppId, AppSlice, Container, Callback, Driver, ReturnCode, Shared};
use kernel::hil::aes::{AESDriver, Client};
use kernel::process::Error;
use kernel::common::take_cell::{MapCell, TakeCell};

pub struct App {
    callback: Option<Callback>,
    buffer: Option<AppSlice<Shared, u8>>,
}

impl Default for App {
    fn default() -> App {
        App {
            callback: None,
            buffer: None,
        }
    }
}


pub struct Encrypt<'a, E: AESDriver + 'a> {
    enc: &'a E,
    apps: Container<App>,
}

impl<'a, E: AESDriver + 'a> Encrypt<'a, E> {
    pub fn new(enc: &'a E, container: Container<App>) -> Encrypt<'a, E> {
        Encrypt {
            enc: enc,
            apps: container,
        }
    }
}

impl<'a, E: AESDriver + 'a> Client for Encrypt<'a, E> {
    fn encrypt_done(&self) -> ReturnCode {
        ReturnCode::SUCCESS
    }

    fn decrypt_done(&self) -> ReturnCode {
        ReturnCode::SUCCESS
    }
}


impl<'a, E: AESDriver> Driver for Encrypt<'a, E> {
    fn allow(&self, appid: AppId, allow_num: usize, slice: AppSlice<Shared, u8>) -> ReturnCode {
        match allow_num {
          e @ 0 ... 2 => {
                self.apps
                    .enter(appid, |app, _| {
                        app.buffer = Some(slice);
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
        ReturnCode::SUCCESS
    }
}

