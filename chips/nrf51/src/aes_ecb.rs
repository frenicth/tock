use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::common::take_cell::TakeCell;
use kernel::hil::aes::{AESDriver, Client};
use kernel::returncode::ReturnCode;
use nvic;
use peripheral_interrupts::NvicIdx;
use peripheral_registers::{AESECB_REGS, AESECB_BASE};
use test;

#[deny(no_mangle_const_items)]

static mut ecb_data: [u8; 48] = [0; 48];
// key 0-16 bytes
// cleartext 16-32 bytes
// ciphertext 32-48 bytes


#[no_mangle]
pub struct AesECB {
    regs: *mut AESECB_REGS,
    client: Cell<Option<&'static Client>>,
}


pub static mut AESECB: AesECB = AesECB::new();

impl AesECB {
    const fn new() -> AesECB {
        AesECB {
            regs: AESECB_BASE as *mut AESECB_REGS,
            client: Cell::new(None),
        }
    }

    pub fn ecb_init(&self) {
        let regs: &mut AESECB_REGS = unsafe { mem::transmute(self.regs) };
        unsafe {
            regs.ECBDATAPTR.set((&ecb_data as *const u8) as u32);
        }
        self.enable_interrupts();
        self.enable_nvic();
    }

    // check components/drivers_nrf/hal/nrf_ecb.c for inspiration :)
    fn encrypt(&self, plaintext: &'static mut [u8]) -> &'static mut [u8] {
        panic!("NOT IMPLEMENTED YET");
        plaintext
    }

    fn decrypt(&self, ciphertext: &'static mut [u8]) -> &'static mut [u8] {
        panic!("NOT IMPLEMENTED YET");
        ciphertext
    }


    fn set_key(&self, key: &'static mut [u8]) -> &'static mut [u8] {
        test::test_aes_ecb_test();
        
        for (i, c) in key.as_ref()[0..16].iter().enumerate() {
            unsafe { ecb_data[i] = *c; }
        }
        // MOVE THIS LATER
        unsafe {
            self.client.get().map(|client| client.set_key_done());
        }
        key
    }


    pub fn handle_interrupt(&self) {
        panic!("NOT IMPLEMENTED YET");

    }

    fn enable_interrupts(&self) {
        // set ENDECB bit 
        let regs: &mut AESECB_REGS = unsafe { mem::transmute(self.regs) };
        regs.INTENSET.set(0x02);
    }

    fn disable_interrupts(&self) {
        let regs: &mut AESECB_REGS = unsafe { mem::transmute(self.regs) };
        regs.INTENSET.set(0x00);
    }

    fn enable_nvic(&self) {
        nvic::enable(NvicIdx::ECB);
    }

    fn disable_nvic(&self) {
        nvic::disable(NvicIdx::ECB);
    }

    pub fn set_client<C: Client>(&self, client: &'static C) {
        // test::test_aes_ecb_test();
        self.client.set(Some(client));
    }
}

// Methods of RadioDummy Trait/Interface and are shared between Capsules and Chips
impl AESDriver for AesECB {
    // This Function is called once Tock is booted
    fn init(&self) -> ReturnCode {
        self.ecb_init();
        ReturnCode::SUCCESS
    }

    fn set_key(&self, key: &'static mut [u8]) -> &'static mut [u8] {
        self.set_key(key)
    }

    // This Function is called once a radio packet is to be sent
    fn encrypt(&self, plaintext: &'static mut [u8]) -> &'static mut [u8] {
        self.encrypt(plaintext)
    }

    // This Function is called once a radio packet is to be sent
    fn decrypt(&self, ciphertext: &'static mut [u8]) -> &'static mut [u8] {
        self.decrypt(ciphertext)
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ECB_Handler() {
    use kernel::common::Queue;
    nvic::disable(NvicIdx::ECB);
    chip::INTERRUPT_QUEUE.as_mut().unwrap().enqueue(NvicIdx::ECB);
}
