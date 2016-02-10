// Automatically generated by tools/nRF51_codegen.py
use common::VolatileCell;

pub const GPIO_BASE: usize = 0x50000000;
pub struct GPIO {
    _pad0: [u8; 1284],
    pub out: VolatileCell<u32>,
    pub outset: VolatileCell<u32>,
    pub outclr: VolatileCell<u32>,
    pub in_: VolatileCell<u32>,
    pub dir: VolatileCell<u32>,
    pub dirset: VolatileCell<u32>,
    pub dirclr: VolatileCell<u32>,
    _pad1: [u8; 480],
    pub pin_cnf: [VolatileCell<u32>; 32],
}