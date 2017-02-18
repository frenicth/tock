use bitfields::*;
use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::common::take_cell::TakeCell;
use kernel::hil::aes::{AESDriver, Client};
use kernel::returncode::ReturnCode;
use nvic;
use peripheral_interrupts::NvicIdx;
use peripheral_registers::{AESCCM_REGS, AESCCM_BASE};

use test;

// maybe make this to a struct later
// byte 0-15 key
// byte 16-24 packet counters
// byte 25-32 IV
static mut CCM_DATA: [u8; 32] = [0; 32];


// byte 0       ;;  Header
// byte 1       ;;  Length
// byte 2       ;;  NOT used
// byte 3-X     ;;  PAYLOAD
// maxpayload 27 bytes
static mut IN_DATA: [u8; 8] = [1, 5, 0, 1, 2, 3, 4, 5];

// byte 0       ;;  Header
// byte 1       ;;  Length+4
// byte 2       ;;  NOT used
// byte 3-X     ;;  Encrypted PAYLOAD
// byte x+4     ;;  MIC
static mut OUT_DATA: [u8; 32] = [0; 32];

// scratchdata for temp usage
static mut TMP: [u8; 32] = [0; 32];


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

    pub fn ccm_init(&self) {
        let regs: &mut AESCCM_REGS = unsafe { mem::transmute(self.regs) };
        // CNFPTR       ;;  datastructure (key, nonce)
        // INPTR        ;;  indata
        // OUTPTR       ;;  outdata
        // SCRATCHDATA  ;;  temporary storage upon key generation
        unsafe {
            regs.CNFPTR.set((&CCM_DATA as *const u8) as u32);
            regs.INPTR.set((&IN_DATA as *const u8) as u32);
            regs.OUTPTR.set((&OUT_DATA as *const u8) as u32);
            regs.SCRATCHPTR.set((&TMP as *const u8) as u32);
        }

    }
    fn set_key(&self, key: &'static mut [u8]) {
        
        for (i, c) in key.as_ref()[0..16].iter().enumerate() {
            unsafe {
                CCM_DATA[i] = *c;
            }
        }
        // MOVE THIS LATER
        unsafe {
            self.client.get().map(|client| client.set_key_done(&mut CCM_DATA[0 .. 16]));
        }
    }

    fn encrypt(&self, pt: &'static mut [u8]) {
        let regs: &mut AESCCM_REGS = unsafe { mem::transmute(self.regs) };


        // panic!("enable {:?}\n", regs.MODE.get());
        if regs.ERROR.get() != 0 {
            panic!("ENCRYPTION ERROR  before CRYPT {}\r\n", regs.ERROR.get());
        }

        // enable aes_ccm
        regs.ENABLE.set(0x02);

        // set encryption mode
        regs.MODE.set(0x00);

        self.enable_nvic();
        self.enable_interrupts();

        regs.ENDKSGEN.set(0);
        regs.ENDCRYPT.set(0);
        regs.KSGEN.set(1);
    }

    fn decrypt(&self, ct: &'static mut [u8]) {
        panic!("DECRYPT NOT IMPLEMENTED YET");
    }

    pub fn handle_interrupt(&self) {
        let regs: &mut AESCCM_REGS = unsafe { mem::transmute(self.regs) };

        if regs.ENDKSGEN.get() == 1 {
            // panic!("ENDKSGEN\n");
            
            // disable endksgen interrupts
            regs.INTENCLR.set(0x01);
            regs.ENDKSGEN.set(0);
            
            // start encryption/decryption
            regs.ENDCRYPT.set(0);
            regs.CRYPT.set(1);
        }

        if regs.ENDCRYPT.get() == 1 {
            // disable endcrypt interrupts
            regs.INTENCLR.set(0x02);
            regs.ENDCRYPT.set(0);

            unsafe {
                // the entire packet is sent to userland atm i.e. header + payload + MIC
                // easy to fix :) but we need to discuss the logic
                self.client.get().map(|client| client.encrypt_done(&mut OUT_DATA));
            }
        }

        if regs.ERROR.get() == 1 { 
            panic!("error AES CCM CRYPT \r\n");
        }

    }

    fn enable_interrupts(&self) {
        let regs: &mut AESCCM_REGS = unsafe { mem::transmute(self.regs) };
        // Enable ENDSKGGEN, ENDSCRYPT and Error Interrupt
        regs.INTENSET.set(1 | 1 << 1 | 1 << 2); // <-> 1 + 2 + 4
    }

    fn disable_interrupts(&self) {
        // let regs: &mut AESCCM_REGS = unsafe { mem::transmute(self.regs) };
        panic!("NOT IMPLEMEMENTED YET\n");
    }

    fn enable_nvic(&self) {
        nvic::enable(NvicIdx::CCM_AAR);
    }

    fn disable_nvic(&self) {
        nvic::disable(NvicIdx::CCM_AAR);
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

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn CCM_AAR_Handler() {
    use kernel::common::Queue;
    nvic::disable(NvicIdx::CCM_AAR);
    chip::INTERRUPT_QUEUE.as_mut().unwrap().enqueue(NvicIdx::CCM_AAR);
}