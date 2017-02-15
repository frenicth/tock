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
pub static CCM_DATA: [u8; 32] = [0; 32];


// byte 0       ;;  Header
// byte 1       ;;  Length
// byte 2       ;;  NOT used
// byte 3-X     ;;  PAYLOAD
// maxpayload 27 bytes
pub static IN_DATA: [u8; 8] = [1, 5, 0, 1, 2, 3, 4, 5];

// byte 0       ;;  Header
// byte 1       ;;  Length+4
// byte 2       ;;  NOT used
// byte 3-X     ;;  Encrypted PAYLOAD
// byte x+4     ;;  MIC
pub static OUT_DATA: [u8; 12] = [1, 9, 0, 1, 2, 3, 4, 5, 0, 0, 0, 0];

// scratchdata for temp usage
pub static TMP: [u8; 32] = [0; 32];


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

        // enable aes_ccm
        regs.ENABLE.set(0x02);

        // CNFPTR       ;;  datastructure (key, nonce)
        // INPTR        ;;  indata
        // OUTPTR       ;;  outdata
        // SCRATCHDATA  ;;  temporary storage upon key generation
        unsafe {
            regs.CNFPTR.set((&CCM_DATA as *const u8) as u32);
            regs.INPTR.set((&IN_DATA as *const u8) as u32);
            regs.OUTPTR.set((&OUT_DATA as *const u8) as u32);
            regs.SHORTS.set((&TMP as *const u8) as u32);
        }


    }
    fn set_key(&self, key: &'static mut [u8]) {
        panic!("SET KEY NOT IMPLEMENTED");
    }

    fn encrypt(&self, pt: &'static mut [u8]) {
        let regs: &mut AESCCM_REGS = unsafe { mem::transmute(self.regs) };

        // panic!("enable {:?}\n", regs.MODE.get());
        if regs.ERROR.get() != 0 {
            panic!("ENCRYPTION ERROR  before CRYPT {}\r\n", regs.ERROR.get());
        }

        // set encryption mode
        regs.MODE.set(0x00);

        regs.ENDKSGEN.set(0);
        regs.KSGEN.set(1);

        while regs.ENDKSGEN.get() == 0 {
            if regs.ERROR.get() != 0 {
                panic!("ENCRYPTION ERROR after KSGEN {}\r\n", regs.ERROR.get());
            }
        }

        regs.ENDCRYPT.set(0);
        regs.CRYPT.set(1);

        while regs.ENDCRYPT.get() == 0 {
            if regs.ERROR.get() != 0 {
                panic!("ENCRYPTION ERROR after CRYPT {}\r\n", regs.ERROR.get());
            }
        }
        panic!("CCM_DATA {:?}\r\n TMP {:?}\r\n IN_DATA {:?}\r\n OUT_DATA {:?}\r\n",
               CCM_DATA,
               TMP,
               IN_DATA,
               OUT_DATA);
    }

    fn decrypt(&self, ct: &'static mut [u8]) {
        panic!("DECRYPT NOT IMPLEMENTED YET");
    }

    fn handle_interrupt(&self) {
        panic!(" HANDLE INTERRUPT NOT IMPLEMENTED YET");
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
        // test::test_aes_ccm();
        // self.set_key(key)
        // DO NOTHING ATM
        ()
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
