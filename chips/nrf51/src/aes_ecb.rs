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
// key 0-15 bytes
// cleartext 16-31 bytes
// ciphertext 32-47 bytes


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
        // self.enable_interrupts();
        // self.enable_nvic();
    }

    // check components/drivers_nrf/hal/nrf_ecb.c for inspiration :)
    fn encrypt(&self, plaintext: &'static mut [u8]) {
        let regs: &mut AESECB_REGS = unsafe { mem::transmute(self.regs) };

        let mut dummy = 0x1000000;

        for (i, c) in plaintext.as_ref()[0..16].iter().enumerate() {
            unsafe {
                ecb_data[i + 16] = *c;
            }
        }

        regs.ENDECB.set(0);
        regs.STARTECB.set(1);

        while regs.ENDECB.get() == 0 {
            if dummy == 0 {
                panic!("NOT ENCRYPTED");
            }
            dummy -= 1;
        }

        // unsafe {
        //     let ct = &ecb_data[0 .. 48];
        //     panic!("{:?}\n", ct);
        // }

        unsafe {
        self.client.get().map(|client| client.encrypt_done(&mut ecb_data[32 .. 48], 16));
        }
    }
    fn decrypt(&self, ciphertext: &'static mut [u8]) {
        panic!("NOT IMPLEMENTED YET");
    }

    fn set_key(&self, key: &'static mut [u8]) {
        test::test_aes_ecb_test();

        for (i, c) in key.as_ref()[0..16].iter().enumerate() {
            unsafe {
                ecb_data[i] = *c;
            }
        }
        // MOVE THIS LATER
        unsafe {
            self.client.get().map(|client| client.set_key_done(&mut ecb_data[0 .. 16], 16));
        }
    }

    pub fn handle_interrupt(&self) {
        panic!("NOT IMPLEMENTED YET");

        let regs: &mut AESECB_REGS = unsafe { mem::transmute(self.regs) };
        if regs.ENDECB.get() == 1 {
            // TODO CALLBACK
        }
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
    fn init(&self) {
        self.ecb_init();
    }

    fn set_key(&self, key: &'static mut [u8], len: u8) {
        self.set_key(key)
    }

    // This Function is called once a radio packet is to be sent
    fn encrypt(&self, plaintext: &'static mut [u8], len: u8) {
        self.encrypt(plaintext)
    }

    // This Function is called once a radio packet is to be sent
    fn decrypt(&self, ciphertext: &'static mut [u8], len: u8) {
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
