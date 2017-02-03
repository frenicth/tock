use core::cell::Cell;
use kernel::hil::radio_nrf51dk::RadioDummy;
use kernel::returncode::ReturnCode;
use kernel::{AppId, Driver, Callback, AppSlice, Shared};
use kernel::common::take_cell::{MapCell, TakeCell};

// FIX THIS ATTRIBUTES LATER
pub struct Radio<'a, R: RadioDummy + 'a> {
    radio: &'a R,
    callback: Cell<Option<Callback>>,
    //tx: TakeCell<&'static mut [u8]>,
    //rx: TakeCell<&'static mut [u8]>,
}

// 'a = lifetime
// R - type Radio
impl<'a, R: RadioDummy + 'a> Radio<'a, R> {
    pub fn new(radio: &'a R) -> Radio<'a, R> {
        Radio {
            radio: radio,
            callback: Cell::new(None),
            //tx: TakeCell::empty(),
            //rx: TakeCell::empty(),
        }
    }

    pub fn capsule_init(&self) {
        // call chips::radio::init()
        self.radio.init()
    }

    // TODO ADD MORE FUNCTIONS
}

// Implementation of the Driver Trait/Interface
impl<'a, R: RadioDummy + 'a> Driver for Radio<'a, R> {
    #[inline(never)]
    #[no_mangle]
    fn command(&self, command_num: usize, data: usize, _: AppId) -> ReturnCode {
        // self.radio.init();
        // call tx / rx HEJSAN
        //self.radio.init();
        self.radio.send();
        self.radio.receive();
        ReturnCode::SUCCESS
    }

    fn subscribe(&self, subscribe_num: usize, callback: Callback) -> ReturnCode {
        // panic!("");
        match subscribe_num {
            // subscribe to all pin interrupts
            // (no affect or reliance on individual pins being configured as interrupts)
            0 => {
                // panic!("");
                //self.callback.set(Some(callback));
                // r0: usize, r1: usize, r2: usize
                // self.callback.get().unwrap().schedule(0, 0, 0);
                ReturnCode::SUCCESS

            }

            // default
            _ => ReturnCode::SUCCESS,
        }
    }


    fn allow(&self, _appid: AppId, allow_num: usize, slice: AppSlice<Shared, u8>) -> ReturnCode {
        panic!("allow num {:?} \t appslice {:?}", allow_num, slice.len());
        ReturnCode::SUCCESS
    }


}
