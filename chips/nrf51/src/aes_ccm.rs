use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::hil::radio_nrf51dk::RadioDummy;
use kernel::returncode::ReturnCode;
use kernel::common::take_cell::TakeCell;
use nvic;
use peripheral_interrupts::NvicIdx;

use gpio;
use kernel::hil::gpio::Pin;
extern crate capsules;
use self::capsules::led::LED;
use peripheral_registers::{RADIO_REGS, RADIO_BASE};

use test;
use bitfields::*;

#[deny(no_mangle_const_items)]


#[no_mangle]
pub struct AesCCM {
    regs: *mut RADIO_REGS,
    tx_buffer: TakeCell<'static, [u8]>,
    rx_buffer: TakeCell<'static, [u8]>,
}

pub static mut AESCCM: Radio = AesCCM::new();

impl Aes {
    #[inline(never)]
    #[no_mangle]
    pub const fn new() -> AesCCM {
        AesEcb {
            regs: RADIO_BASE as *mut RADIO_REGS,
            tx_buffer: TakeCell::empty(),
            rx_buffer : TakeCell::empty(),
        }
    }

    pub fn config(&self) {
        panic!("NOT IMPLEMENTED YET");
        // let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };

    }
    
    #[inline(never)]
    #[no_mangle]
    pub fn encrypt(&self, dest: u16, tx_data: &'static mut [u8], tx_len: u8) {

        
        panic!("NOT IMPLEMENTED YET");
        
    }

    #[inline(never)]
    #[no_mangle]
    pub fn decrypt(&self) {
        panic!("NOT IMPLEMENTED YET");
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };

    }

    #[inline(never)]
    #[no_mangle]
    pub fn handle_interrupt(&self) {
        panic!("NOT IMPLEMENTED YET");
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };

    }


    pub fn enable_interrupts(&self) {
        panic!("NOT IMPLEMENTED YET");
        // let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
    }

    pub fn disable_interrupts(&self) {
        panic!("NOT IMPLEMENTED YET");
    }

    pub fn enable_nvic(&self) {
        panic!("NOT IMPLEMENTED YET");
        // nvic::enable(NvicIdx::RADIO);
    }

    pub fn disable_nvic(&self) {
        panic!("NOT IMPLEMENTED YET");
        // nvic::disable(NvicIdx::RADIO);
    }

}
// Methods of RadioDummy Trait/Interface and are shared between Capsules and Chips
impl AesCCMDriver for AesCCM {

    // This Function is called once Tock is booted
    fn init(&self) {
        // panic!();
        self.config()
    }

    // This Function is called once a radio packet is to be sent
    fn encrypt(&self) {
        // self.tx(0, 0, 0);
        panic!("NOT USED ATM");
    }

    // This Function is called once a radio packet is to be sent
    fn decrypt(&self) {
    }
}


// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn CCM_AAR_Handler() {
//     use kernel::common::Queue;
//     nvic::disable(NvicIdx::AESECB);
//     chip::INTERRUPT_QUEUE.as_mut().unwrap().enqueue(NvicIdx::AESECB);
// }
