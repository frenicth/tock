#![feature(const_fn)]
#![no_std]

extern crate kernel;
extern crate sam4l;

pub mod button;
pub mod console;
pub mod fm25cl;
pub mod gpio;
pub mod isl29035;
pub mod led;
pub mod nrf51822_serialization;
pub mod timer;
pub mod tmp006;
pub mod si7021;
pub mod spi;
pub mod virtual_alarm;
pub mod virtual_i2c;
pub mod virtual_spi;
pub mod adc;
pub mod i2c_master_slave_driver;
pub mod lps25hb;
