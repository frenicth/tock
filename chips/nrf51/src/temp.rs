use bitfields::*;
use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::common::take_cell::TakeCell;
use kernel::hil::temp::{TempDriver, Client};
use kernel::returncode::ReturnCode;
use nvic;
use peripheral_interrupts::NvicIdx;
use peripheral_registers::{TEMP_REGS, TEMP_BASE};

use test;



#[deny(no_mangle_const_items)]
#[no_mangle]
pub struct Temp {
    regs: *mut TEMP_REGS,
    client: Cell<Option<&'static Client>>,
}

pub static mut TEMP: Temp = Temp::new();

impl Temp {
    const fn new() -> Temp {
        Temp  {
            regs: TEMP_BASE as *mut TEMP_REGS,
            client: Cell::new(None),
        }
    }

    pub fn init_temp(&self) {
        ()
    }

    fn measure(&self) {
        let regs: &mut TEMP_REGS = unsafe { mem::transmute(self.regs) };
        test::test_temp();
       
        regs.DATARDY.set(0);
        regs.START.set(1);

        while regs.DATARDY.get() == 0 {}

        let temp = regs.TEMP.get()/4;

        regs.STOP.set(1);
        self.client.get().map(|client| client.measurement_done(temp as usize));

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
impl TempDriver for Temp {
    // This Function is called once Tock is booted
    fn init(&self) {
        self.init_temp()
    }
    
    fn take_measurement(&self) {
        self.measure()
    }
}
// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn CCM_AAR_Handler() {
//     use kernel::common::Queue;
//     nvic::disable(NvicIdx::AESECB);
//     chip::INTERRUPT_QUEUE.as_mut().unwrap().enqueue(NvicIdx::AESECB);
// }
