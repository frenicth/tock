use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::returncode::ReturnCode;
use kernel::common::take_cell::TakeCell;
use nvic;
use peripheral_interrupts::NvicIdx;

use peripheral_registers::{AESCCM_REGS, AESCCM_BASE};

use test;
use bitfields::*;

#[deny(no_mangle_const_items)]


#[no_mangle]
pub struct AesCCM {
    regs: *mut AESCCM_REGS,
}

pub static mut AESCCM: AesCCM = AesCCM::new();

impl AesCCM {
    const fn new() -> AesCCM {
        AesCCM {
            regs: AESCCM_BASE as *mut AESCCM_REGS,
        }
    }

    fn config(&self) {
        panic!("NOT IMPLEMENTED YET");
        // let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };

    }
    
    fn encrypt(&self, dest: u16, tx_data: &'static mut [u8], tx_len: u8) {
        panic!("NOT IMPLEMENTED YET");
        
    }

    fn decrypt(&self) {
        panic!("NOT IMPLEMENTED YET");
    }

    fn handle_interrupt(&self) {
        panic!("NOT IMPLEMENTED YET");
    }


    fn enable_interrupts(&self) {
        panic!("NOT IMPLEMENTED YET");
        // let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
    }

    fn disable_interrupts(&self) {
        panic!("NOT IMPLEMENTED YET");
    }

    fn enable_nvic(&self) {
        panic!("NOT IMPLEMENTED YET");
        // nvic::enable(NvicIdx::RADIO);
    }

    fn disable_nvic(&self) {
        panic!("NOT IMPLEMENTED YET");
        // nvic::disable(NvicIdx::RADIO);
    }

}
// Methods of RadioDummy Trait/Interface and are shared between Capsules and Chips
// impl AesCCMDriver for AesCCM {
//
//     // This Function is called once Tock is booted
//     fn init(&self) {
//         // panic!();
//         self.config()
//     }
//
//     // This Function is called once a radio packet is to be sent
//     fn encrypt(&self) {
//         // self.tx(0, 0, 0);
//         panic!("NOT USED ATM");
//     }
//
//     // This Function is called once a radio packet is to be sent
//     fn decrypt(&self) {
//     }
// }


// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn CCM_AAR_Handler() {
//     use kernel::common::Queue;
//     nvic::disable(NvicIdx::AESECB);
//     chip::INTERRUPT_QUEUE.as_mut().unwrap().enqueue(NvicIdx::AESECB);
// }
