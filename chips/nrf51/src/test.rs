use kernel::common::VolatileCell;
use peripheral_registers::{RADIO_REGS, RADIO_BASE, AESECB_BASE, AESECB_REGS, AESCCM_REGS, AESCCM_BASE};
use core::mem;

static mut ptr: *mut u32 = RADIO_BASE as *mut u32;
static mut aes_ecb_ptr: *mut u32 = AESECB_BASE as *mut u32;
static mut aes_ccm_ptr: *mut u32 = AESCCM_BASE as *mut u32;


pub fn test_radio_regs() {
    let regs: &mut RADIO_REGS = unsafe { mem::transmute(ptr)};
    assert_eq!(0x40001000 as * const  VolatileCell<u32>, &regs.TXEN as *const VolatileCell<u32>);
    assert_eq!(0x40001004 as * const  VolatileCell<u32>, &regs.RXEN as *const VolatileCell<u32>);
    assert_eq!(0x40001008 as * const  VolatileCell<u32>, &regs.START as *const VolatileCell<u32>);
    assert_eq!(0x4000100c as * const  VolatileCell<u32>, &regs.STOP as *const VolatileCell<u32>);
    assert_eq!(0x40001010 as * const  VolatileCell<u32>, &regs.DISABLE as *const VolatileCell<u32>);
    assert_eq!(0x40001014 as * const  VolatileCell<u32>, &regs.RSSISTART as *const VolatileCell<u32>);
    assert_eq!(0x40001018 as * const  VolatileCell<u32>, &regs.RSSISTOP as *const VolatileCell<u32>);
    assert_eq!(0x4000101c as * const  VolatileCell<u32>, &regs.BCSTART as *const VolatileCell<u32>);
    assert_eq!(0x40001020 as * const  VolatileCell<u32>, &regs.BCSTOP as *const VolatileCell<u32>);
    assert_eq!(0x40001100 as * const  VolatileCell<u32>, &regs.READY as *const VolatileCell<u32>);
    assert_eq!(0x40001104 as * const  VolatileCell<u32>, &regs.ADDRESS as *const VolatileCell<u32>);
    assert_eq!(0x40001108 as * const  VolatileCell<u32>, &regs.PAYLOAD as *const VolatileCell<u32>);
    assert_eq!(0x4000110c as * const  VolatileCell<u32>, &regs.END as *const VolatileCell<u32>);
    assert_eq!(0x40001110 as * const  VolatileCell<u32>, &regs.DISABLED as *const VolatileCell<u32>);
    assert_eq!(0x40001114 as * const  VolatileCell<u32>, &regs.DEVMATCH as *const VolatileCell<u32>);
    assert_eq!(0x40001118 as * const  VolatileCell<u32>, &regs.DEVMISS as *const VolatileCell<u32>);
    assert_eq!(0x4000111c as * const  VolatileCell<u32>, &regs.RSSIEND as *const VolatileCell<u32>);
    assert_eq!(0x40001128 as * const  VolatileCell<u32>, &regs.BCMATCH as *const VolatileCell<u32>);
    assert_eq!(0x40001304 as * const  VolatileCell<u32>, &regs.INTENSET as *const VolatileCell<u32>);
    assert_eq!(0x40001308 as * const  VolatileCell<u32>, &regs.INTENCLR as *const VolatileCell<u32>);
    assert_eq!(0x40001400 as * const  VolatileCell<u32>, &regs.CRCSTATUS as *const VolatileCell<u32>);
    assert_eq!(0x40001408 as * const  VolatileCell<u32>, &regs.RXMATCH as *const VolatileCell<u32>);
    assert_eq!(0x4000140c as * const  VolatileCell<u32>, &regs.RXCRC as *const VolatileCell<u32>);
    assert_eq!(0x40001410 as * const  VolatileCell<u32>, &regs.DAI as *const VolatileCell<u32>);
    assert_eq!(0x40001504 as * const  VolatileCell<u32>, &regs.PACKETPTR as *const VolatileCell<u32>);
    assert_eq!(0x40001508 as * const  VolatileCell<u32>, &regs.FREQEUNCY as *const VolatileCell<u32>);
    assert_eq!(0x4000150c as * const  VolatileCell<u32>, &regs.TXPOWER as *const VolatileCell<u32>);
    assert_eq!(0x40001510 as * const  VolatileCell<u32>, &regs.MODE as *const VolatileCell<u32>);
    assert_eq!(0x40001514 as * const  VolatileCell<u32>, &regs.PCNF0 as *const VolatileCell<u32>);
    assert_eq!(0x40001518 as * const  VolatileCell<u32>, &regs.PCNF1 as *const VolatileCell<u32>);
    assert_eq!(0x4000151c as * const  VolatileCell<u32>, &regs.BASE0 as *const VolatileCell<u32>);
    assert_eq!(0x40001520 as * const  VolatileCell<u32>, &regs.BASE1 as *const VolatileCell<u32>);
    assert_eq!(0x40001524 as * const  VolatileCell<u32>, &regs.PREFIX0 as *const VolatileCell<u32>);
    assert_eq!(0x40001528 as * const  VolatileCell<u32>, &regs.PREFIX1 as *const VolatileCell<u32>);
    assert_eq!(0x4000152c as * const  VolatileCell<u32>, &regs.TXADDRESS as *const VolatileCell<u32>);
    assert_eq!(0x40001530 as * const  VolatileCell<u32>, &regs.RXADDRESSES as *const VolatileCell<u32>);
    assert_eq!(0x40001534 as * const  VolatileCell<u32>, &regs.CRCCNF as *const VolatileCell<u32>);
    assert_eq!(0x40001538 as * const  VolatileCell<u32>, &regs.CRCPOLY as *const VolatileCell<u32>);
    assert_eq!(0x4000153c as * const  VolatileCell<u32>, &regs.CRCINIT as *const VolatileCell<u32>);
    assert_eq!(0x40001540 as * const  VolatileCell<u32>, &regs.TEST as *const VolatileCell<u32>);
    assert_eq!(0x40001544 as * const  VolatileCell<u32>, &regs.TIFS as *const VolatileCell<u32>);
    assert_eq!(0x40001548 as * const  VolatileCell<u32>, &regs.RSSISAMPLE as *const VolatileCell<u32>);
    assert_eq!(0x40001550 as * const  VolatileCell<u32>, &regs.STATE as *const VolatileCell<u32>);
    assert_eq!(0x40001554 as * const  VolatileCell<u32>, &regs.DATAWHITEIV as *const VolatileCell<u32>);
    assert_eq!(0x40001600 as * const  VolatileCell<u32>, &regs.DAB0 as *const VolatileCell<u32>);
    assert_eq!(0x40001604 as * const  VolatileCell<u32>, &regs.DAB1 as *const VolatileCell<u32>);
    assert_eq!(0x40001608 as * const  VolatileCell<u32>, &regs.DAB2 as *const VolatileCell<u32>);
    assert_eq!(0x4000160c as * const  VolatileCell<u32>, &regs.DAB3 as *const VolatileCell<u32>);
    assert_eq!(0x40001610 as * const  VolatileCell<u32>, &regs.DAB4 as *const VolatileCell<u32>);
    assert_eq!(0x40001614 as * const  VolatileCell<u32>, &regs.DAB5 as *const VolatileCell<u32>);
    assert_eq!(0x40001618 as * const  VolatileCell<u32>, &regs.DAB6 as *const VolatileCell<u32>);
    assert_eq!(0x4000161c as * const  VolatileCell<u32>, &regs.DAB7 as *const VolatileCell<u32>);
    assert_eq!(0x40001620 as * const  VolatileCell<u32>, &regs.DAP0 as *const VolatileCell<u32>);
    assert_eq!(0x40001624 as * const  VolatileCell<u32>, &regs.DAP1 as *const VolatileCell<u32>);
    assert_eq!(0x40001628 as * const  VolatileCell<u32>, &regs.DAP2 as *const VolatileCell<u32>);
    assert_eq!(0x4000162c as * const  VolatileCell<u32>, &regs.DAP3 as *const VolatileCell<u32>);
    assert_eq!(0x40001630 as * const  VolatileCell<u32>, &regs.DAP4 as *const VolatileCell<u32>);
    assert_eq!(0x40001634 as * const  VolatileCell<u32>, &regs.DAP5 as *const VolatileCell<u32>);
    assert_eq!(0x40001638 as * const  VolatileCell<u32>, &regs.DAP6 as *const VolatileCell<u32>);
    assert_eq!(0x4000163c as * const  VolatileCell<u32>, &regs.DAP7 as *const VolatileCell<u32>);
    assert_eq!(0x40001640 as * const  VolatileCell<u32>, &regs.DACNF as *const VolatileCell<u32>);
    assert_eq!(0x40001724 as * const  VolatileCell<u32>, &regs.OVERRIDE0 as *const VolatileCell<u32>);
    assert_eq!(0x40001728 as * const  VolatileCell<u32>, &regs.OVERRIDE1 as *const VolatileCell<u32>);
    assert_eq!(0x4000172c as * const  VolatileCell<u32>, &regs.OVERRIDE2 as *const VolatileCell<u32>);
    assert_eq!(0x40001730 as * const  VolatileCell<u32>, &regs.OVERRIDE3 as *const VolatileCell<u32>);
    assert_eq!(0x40001734 as * const  VolatileCell<u32>, &regs.OVERRIDE4 as *const VolatileCell<u32>);
    assert_eq!(0x40001ffc as * const  VolatileCell<u32>, &regs.POWER as *const VolatileCell<u32>);
}



pub fn test_aes_ecb_test() {
    let regs: &mut AESECB_REGS = unsafe { mem::transmute(aes_ecb_ptr)};
    assert_eq!(0x4000E000 as * const  VolatileCell<u32>, &regs.STARTECB as *const VolatileCell<u32>);
    // panic!("test_aes_ecb");
    assert_eq!(0x4000E004 as * const  VolatileCell<u32>, &regs.STOPECB as *const VolatileCell<u32>);
    assert_eq!(0x4000E100 as * const  VolatileCell<u32>, &regs.ENDECB as *const VolatileCell<u32>);
    assert_eq!(0x4000E104 as * const  VolatileCell<u32>, &regs.ERRORECB as *const VolatileCell<u32>);
    assert_eq!(0x4000E304 as * const  VolatileCell<u32>, &regs.INTENSET as *const VolatileCell<u32>);
    assert_eq!(0x4000E308 as * const  VolatileCell<u32>, &regs.INTENCLR as *const VolatileCell<u32>);
    assert_eq!(0x4000E504 as * const  VolatileCell<u32>, &regs.ECBDATAPTR as *const VolatileCell<u32>);
}

pub fn test_aes_ccm() {
    let regs: &mut AESCCM_REGS = unsafe { mem::transmute(aes_ccm_ptr)};
    assert_eq!(0x4000F000 as * const  VolatileCell<u32>, &regs.KSGEN as *const VolatileCell<u32>);
    assert_eq!(0x4000F004 as * const  VolatileCell<u32>, &regs.CRYPT as *const VolatileCell<u32>);
    assert_eq!(0x4000F008 as * const  VolatileCell<u32>, &regs.STOP as *const VolatileCell<u32>);
    assert_eq!(0x4000F100 as * const  VolatileCell<u32>, &regs.ENDKSGEN as *const VolatileCell<u32>);
    assert_eq!(0x4000F104 as * const  VolatileCell<u32>, &regs.ENDCRYPT as *const VolatileCell<u32>);
    assert_eq!(0x4000F108 as * const  VolatileCell<u32>, &regs.ERROR as *const VolatileCell<u32>);
    assert_eq!(0x4000F200 as * const  VolatileCell<u32>, &regs.SHORTS as *const VolatileCell<u32>);
    assert_eq!(0x4000F304 as * const  VolatileCell<u32>, &regs.INTENSET as *const VolatileCell<u32>);
    assert_eq!(0x4000F308 as * const  VolatileCell<u32>, &regs.INTENCLR as *const VolatileCell<u32>);
    assert_eq!(0x4000F400 as * const  VolatileCell<u32>, &regs.MICSTATUS as *const VolatileCell<u32>);
    assert_eq!(0x4000F500 as * const  VolatileCell<u32>, &regs.ENABLE as *const VolatileCell<u32>);
    assert_eq!(0x4000F504 as * const  VolatileCell<u32>, &regs.MODE as *const VolatileCell<u32>);
    assert_eq!(0x4000F508 as * const  VolatileCell<u32>, &regs.CNFPTR as *const VolatileCell<u32>);
    assert_eq!(0x4000F50c as * const  VolatileCell<u32>, &regs.INPTR as *const VolatileCell<u32>);
    assert_eq!(0x4000F510 as * const  VolatileCell<u32>, &regs.OUTPTR as *const VolatileCell<u32>);
    assert_eq!(0x4000F514 as * const  VolatileCell<u32>, &regs.SCRATCHPTR as *const VolatileCell<u32>);
}

