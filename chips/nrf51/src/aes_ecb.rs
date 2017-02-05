use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::returncode::ReturnCode;
use kernel::common::take_cell::TakeCell;
use nvic;
use peripheral_interrupts::NvicIdx;

use peripheral_registers::{AESECB_REGS, AESECB_BASE};


#[deny(no_mangle_const_items)]


#[no_mangle]
pub struct AesECB {
    regs: *mut AESECB_REGS,
}

pub static mut AESECB: AesECB = AesECB::new();

impl AesECB {
    #[inline(never)]
    #[no_mangle]
    pub const fn new() -> AesECB {
        AesECB {
            regs: AESECB_BASE as *mut AESECB_REGS,
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

    }

    // pub fn handle_interrupt(&self) {
    //     panic!("NOT IMPLEMENTED YET");
    //
    // }
    //
    //
    // pub fn enable_interrupts(&self) {
    //     panic!("NOT IMPLEMENTED YET");
    //     // let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
    // }
    //
    // pub fn disable_interrupts(&self) {
    //     panic!("NOT IMPLEMENTED YET");
    // }
    //
    // pub fn enable_nvic(&self) {
    //     panic!("NOT IMPLEMENTED YET");
    //     // nvic::enable(NvicIdx::RADIO);
    // }
    //
    // pub fn disable_nvic(&self) {
    //     panic!("NOT IMPLEMENTED YET");
    //     // nvic::disable(NvicIdx::RADIO);
    // }

}

// Methods of RadioDummy Trait/Interface and are shared between Capsules and Chips
// impl AesEcbDriver for Aes {
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
// pub unsafe extern "C" fn ECB_Handler() {
//     use kernel::common::Queue;
//     nvic::disable(NvicIdx::AESECB);
//     chip::INTERRUPT_QUEUE.as_mut().unwrap().enqueue(NvicIdx::AESECB);
// }
