use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::returncode::ReturnCode;
use kernel::common::take_cell::TakeCell;
use nvic;
use peripheral_interrupts::NvicIdx;
use kernel::hil::aes::{AESDriver, Client};
use peripheral_registers::{AESCCM_REGS, AESCCM_BASE};

use test;
use bitfields::*;

#[deny(no_mangle_const_items)]


#[no_mangle]
pub struct AesCCM {
    regs: *mut AESCCM_REGS,
    client: Cell<Option<&'static Client>>,
}

pub static mut AESCCM: AesCCM = AesCCM::new();

impl AesCCM {
    const fn new() -> AesCCM {
        AesCCM {
            regs: AESCCM_BASE as *mut AESCCM_REGS,
            client: Cell::new(None),
        }
    }

    fn ccm_init(&self) {
        panic!("NOT IMPLEMENTED YET");
        // let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };

    }
    
    fn encrypt(&self, pt: &'static mut [u8]) {
        panic!("NOT IMPLEMENTED YET");
        
    }

    fn decrypt(&self, ct: &'static mut [u8]) {
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

    pub fn set_client<C: Client>(&self, client: &'static C) {
        // test::test_aes_ecb_test();
        self.client.set(Some(client));
    }
}
// Methods of RadioDummy Trait/Interface and are shared between Capsules and Chips
impl AESDriver for AesCCM {
    // This Function is called once Tock is booted
    fn init(&self) {
        self.ccm_init()
    }

    fn set_key(&self, key: &'static mut [u8]) {
        self.set_key(key)
    }

    // This Function is called once a radio packet is to be sent
    fn encrypt(&self, plaintext: &'static mut [u8]) {
        self.encrypt(plaintext)
    }

    // This Function is called once a radio packet is to be sent
    fn decrypt(&self, ciphertext: &'static mut [u8]) {
        self.decrypt(ciphertext)
    }
}
// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn CCM_AAR_Handler() {
//     use kernel::common::Queue;
//     nvic::disable(NvicIdx::AESECB);
//     chip::INTERRUPT_QUEUE.as_mut().unwrap().enqueue(NvicIdx::AESECB);
// }
