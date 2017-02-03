use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::hil::radio_nrf51dk::RadioDummy;
use kernel::returncode::ReturnCode;
use kernel::common::take_cell::TakeCell;
use nvic;
use peripheral_interrupts::NvicIdx;

use gpio;
//extern crate nrf51;
use kernel::hil::gpio::Pin;
extern crate capsules;
use self::capsules::led::LED;



// fill-in according to datasheet
// definies from nrf51_bitfields.h are copied into bluetooth.txt
// we probably don't need all
#[no_mangle]
#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct Registers {
    pub TXEN: VolatileCell<u32>,                    // 0x000 ---> 0x004
    pub RXEN: VolatileCell<u32>,                    // 0x004 ---> 0x008
    pub START: VolatileCell<u32>,                   // 0x008 ---> 0x00c
    pub STOP: VolatileCell<u32>,                    // 0x00c ---> 0x010
    pub DISABLE: VolatileCell<u32>,                 // 0x010 ---> 0x014
    pub RSSISTART: VolatileCell<u32>,               // 0x014 ---> 0x018
    pub RSSISTOP: VolatileCell<u32>,              // 0x018 ---> 0x01c
    pub BCSTART: VolatileCell<u32>,                 // 0x01c ---> 0x020
    pub BCSTOP: VolatileCell<u32>,                  // 0x020 ---> 0x024
    _reserved1: [u32; 55],                          // 0x024 ---> 0x100
    pub READY: VolatileCell<u32>,                   // 0x100 ---> 0x104
    pub ADDRESS: VolatileCell<u32>,                 // 0x104 ---> 0x108
    pub PAYLOAD: VolatileCell<u32>,                 // 0x108 ---> 0x10c
    pub END: VolatileCell<u32>,                     // 0x10c ---> 0x110
    pub DISABLED: VolatileCell<u32>,                // 0x110 ---> 0x114
    pub DEVMATCH: VolatileCell<u32>,                // 0x114 ---> 0x118
    pub DEVMISS: VolatileCell<u32>,                 // 0x118 ---> 0x11c
    pub RSSIEND: VolatileCell<u32>,                 // 0x11c -->  0x120
    _reserved2: [u32; 2],                           // 0x120 ---> 0x128
    pub BCMATCH: VolatileCell<u32>,                 // 0x128 ---> 0x12c
    _reserved3: [u32; 53],                          // 0x12c ---> 0x200
    pub SHORTS: VolatileCell<u32>,                  // 0x200 ---> 0x204
    _reserved4: [u32; 64],                          // 0x204 ---> 0x304
    pub INTENSET: VolatileCell<u32>,                // 0x304 ---> 0x308
    pub INTENCLR: VolatileCell<u32>,                // 0x308 ---> 0x30c
    _reserved5: [u32; 61],                          // 0x30c ----> 0x400
    pub CRCSTATUS: VolatileCell<u32>,               // 0x400 - 404
    _reserved6: [u32; 1],                           // 0x404 - 0x408
    pub RXMATCH: VolatileCell<u32>,                 // 0x408 - 0x40c
    pub RXCRC: VolatileCell<u32>,                   // 0x40c - 0x410
    pub DAI: VolatileCell<u32>,                     // 0x410 - 0x414
    _reserved7: [u32; 60],                          // 0x414 - 0x504
    pub PACKETPTR: VolatileCell<u32>,               // 0x504 - 0x508
    pub FREQEUNCY: VolatileCell<u32>,               // 0x508 - 0x50c
    pub TXPOWER: VolatileCell<u32>,                 // 0x50c - 0x510
    pub MODE: VolatileCell<u32>,                    // 0x510 - 0x514
    pub PCNF0: VolatileCell<u32>,                   // 0x514 - 0x518
    pub PCNF1: VolatileCell<u32>,                   // 0x518 - 0x51c
    pub BASE0: VolatileCell<u32>,                   // 0x51c - 0x520
    pub BASE1: VolatileCell<u32>,                   // 0x520 - 0x524
    pub PREFIX0: VolatileCell<u32>,                 // 0x524 - 0x528
    pub PREFIX1: VolatileCell<u32>,                 // 0x528 - 0x52c
    pub TXADDRESS: VolatileCell<u32>,               // 0x52c - 0x530
    pub RXADDRESSES: VolatileCell<u32>,             // 0x530 - 0x534
    pub CRCCNF: VolatileCell<u32>,                  // 0x534 - 0x538
    pub CRCPOLY: VolatileCell<u32>,                 // 0x538 - 0x53c
    pub CRCINIT: VolatileCell<u32>,                 // 0x53c - 0x540
    pub TEST: VolatileCell<u32>,                    // 0x540 - 0x544
    pub TIFS: VolatileCell<u32>,                    // 0x544 - 0x548
    pub RSSISAMPLE: VolatileCell<u32>,              // 0x548 - 0x54c
    _reserved8: [u32; 1],                           // 0x54c - 0x550
    pub STATE: VolatileCell<u32>,                   // 0x550 - 0x554
    pub DATAWHITEIV: VolatileCell<u32>,             // 0x554 - 0x558
    _reserved9: [u32; 2],                           // 0x558 - 0x560
    pub BCC: VolatileCell<u32>,                     // 0x560 - 0x564
    _reserved10: [u32; 39],                         // 0x560 - 0x600
    pub DAB0: VolatileCell<u32>,                    // 0x600 - 0x604
    pub DAB1: VolatileCell<u32>,                    // 0x604 - 0x608
    pub DAB2: VolatileCell<u32>,                    // 0x608 - 0x60c
    pub DAB3: VolatileCell<u32>,                    // 0x60c - 0x610
    pub DAB4: VolatileCell<u32>,                    // 0x610 - 0x614
    pub DAB5: VolatileCell<u32>,                    // 0x614 - 0x618
    pub DAB6: VolatileCell<u32>,                    // 0x618 - 0x61c
    pub DAB7: VolatileCell<u32>,                    // 0x61c - 0x620
    pub DAP0: VolatileCell<u32>,                    // 0x620 - 0x624
    pub DAP1: VolatileCell<u32>,                    // 0x624 - 0x628
    pub DAP2: VolatileCell<u32>,                    // 0x628 - 0x62c
    pub DAP3: VolatileCell<u32>,                    // 0x62c - 0x630
    pub DAP4: VolatileCell<u32>,                    // 0x630 - 0x634
    pub DAP5: VolatileCell<u32>,                    // 0x634 - 0x638
    pub DAP6: VolatileCell<u32>,                    // 0x638 - 0x63c
    pub DAP7: VolatileCell<u32>,                    // 0x63c - 0x640
    pub DACNF: VolatileCell<u32>,                   // 0x640 - 0x644
    _reserved11: [u32; 56],                         // 0x644 - 0x724
    pub OVERRIDE0: VolatileCell<u32>,               // 0x724 - 0x728
    pub OVERRIDE1: VolatileCell<u32>,               // 0x728 - 0x72c
    pub OVERRIDE2: VolatileCell<u32>,               // 0x72c - 0x730
    pub OVERRIDE3: VolatileCell<u32>,               // 0x730 - 0x734
    pub OVERRIDE4: VolatileCell<u32>,               // 0x734 - 0x738
    _reserved12: [u32; 561],                        // 0x738 - 0x724
    pub POWER: VolatileCell<u32>,                   // 0xFFC - 0x1000
}

const PACKET0_S1_SIZE: u32 = 0; // S1 size in bits
const PACKET0_S0_SIZE: u32 = 0;
const PACKET0_PAYLOAD_SIZE: u32 = 0;

const PACKET1_BASE_ADDRESS_LENGTH: u32 = 4;
const PACKET1_STATIC_LENGTH: u32 = 1;
const PACKET1_PAYLOAD_SIZE: u32 = 1;

const RADIO_PCNF0_S0LEN_Pos: u32 = 8;
const RADIO_PCNF0_S1LEN_Pos: u32 = 16;
const RADIO_PCNF0_LFLEN_Pos: u32 = 0;

const RADIO_PCNF1_WHITEEN_Disabled: u32 = 0;
const RADIO_PCNF1_WHITEEN_Enabled: u32 = 1;
const RADIO_PCNF1_WHITEEN_Pos: u32 = 25;

const RADIO_PCNF1_BALEN_Pos: u32 = 16;
const RADIO_PCNF1_STATLEN_Pos: u32 = 8;
const RADIO_PCNF1_MAXLEN_Pos: u32 = 0;

const RADIO_PCNF1_ENDIAN_Pos: u32 = 24;
const RADIO_PCNF1_ENDIAN_Big: u32 = 1;


#[deny(no_mangle_const_items)]
// addresses from 0x40001000 - 0x40002000
const RADIO_BASE: u32 = 0x40001000;

static mut tx_buf: [u32; 16] = [3, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
static mut rx_buf: [u32; 16] = [0x00; 16];

#[no_mangle]
pub struct Radio {
    regs: *mut Registers,
    // tx_buffer: TakeCell<&'static mut [u8]>,
    // RX BUFFER
    // TX BUFFER
}

pub static mut RADIO: Radio = Radio::new();


impl Radio {
    #[inline(never)]
    #[no_mangle]
    pub const fn new() -> Radio {
        Radio {
            regs: RADIO_BASE as *mut Registers,
            // nvic: NvicIdx::Radio
            // tx_buffer : TakeCell::empty(),
        }
    }
    // TODO  DEFINE METHODS FOR LOW-LEVEL INIT

    fn turnOnLeds(&self) {

       unsafe { let led0 = &gpio::PORT[21];
       led0.make_output();
       led0.toggle();
       }
    }

    fn test(&self) {

        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };
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

    pub fn config(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };

        self.test();

        // reset and enable power
        regs.POWER.set(0);
        regs.POWER.set(1);

        // set tx power +4 dBm
        regs.TXPOWER.set(0x04);

        // set channel frequency
        regs.FREQEUNCY.set(7);

        // DATAIV
        regs.DATAWHITEIV.set(7);

        // set channel rate,  3 - BLE 1MBIT/s
        regs.MODE.set(3);


        // according BLE standard
        regs.PREFIX0.set(0x8e);
        regs.BASE0.set(0x89bed500);

        // Use logical address 0 (prefix0 + base0) = 0x8E89BED6 when transmitting and receiving
        regs.TXADDRESS.set(0x00);
        regs.RXADDRESSES.set(0x01);
        regs.RXMATCH.set(0x00);


        // checkout this link if this doesn't work
        // https://devzone.nordicsemi.com/question/396/is-it-possible-multiytes-sendingreceiving-in-one-packet-with-rf-radio/
        // maybe this as well
        // https://devzone.nordicsemi.com/question/3315/the-communication-of-nrf51822-and-nrf24l01p/

        const PACKET_LENGTH_FIELD_SIZE: u32 = 0;
        const PACKET_PAYLOAD_MAXSIZE: u32 = 64;
        const PACKET_BASE_ADDRESS_LENGTH: u32 = 4;
        const PACKET_STATIC_LENGTH: u32 = 64;

        regs.PCNF0.set(
            (PACKET0_S1_SIZE << RADIO_PCNF0_S1LEN_Pos) |
            (PACKET0_S0_SIZE << RADIO_PCNF0_S0LEN_Pos) |
            (PACKET_LENGTH_FIELD_SIZE << RADIO_PCNF0_LFLEN_Pos)
            );

        // Packet configuration
        regs.PCNF1.set(
            (RADIO_PCNF1_WHITEEN_Disabled << RADIO_PCNF1_WHITEEN_Pos) |
            (RADIO_PCNF1_ENDIAN_Big       << RADIO_PCNF1_ENDIAN_Pos)  |
            (PACKET_BASE_ADDRESS_LENGTH   << RADIO_PCNF1_BALEN_Pos)   |
            (PACKET_STATIC_LENGTH         << RADIO_PCNF1_STATLEN_Pos) |
            (PACKET_PAYLOAD_MAXSIZE       << RADIO_PCNF1_MAXLEN_Pos)
            );

        // Buffer configuration
        self.set_tx_buffer();

        // CRC Config
        regs.CRCCNF.set(0);         // DON'T USE CRC
        regs.CRCINIT.set(0x00555555);        // INIT CRC Value
        regs.CRCPOLY.set(0x0000065B);        // POLYNOMIAL

        self.enable_interrupts();
        self.enable_nvic();
    }

    fn set_tx_buffer(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };
        unsafe {
            regs.PACKETPTR.set( (&tx_buf as *const u32) as u32);
        }
    }

    fn set_rx_buffer(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };
        unsafe {
            regs.PACKETPTR.set( (&rx_buf as *const u32) as u32);
        }
    }

    #[inline(never)]
    #[no_mangle]
    pub fn tx(&self) {

        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };

        // ISR
        //self.enable_interrupts();
        //self.enable_nvic();

        // panic!("INTENSET {:?}", regs.INTENSET.get());
        regs.READY.set(0);

        // TX ENABLE
        regs.TXEN.set(1);

        //self.set_tx_buffer();

        // // Blocking Dummy Loop
        // while regs.READY.get() == 0 {}
        //
        // regs.READY.set(0);
        // regs.END.set(0);
        //
        // // START RADIO
        // regs.START.set(1);
        //
        // // Address Event
        // while regs.PAYLOAD.get() == 0 {}
        //
        // // Wait Until The Tranmission is Finished
        // while regs.END.get() == 0 {}
        //
        // regs.DISABLE.set(1);
    }

    #[inline(never)]
    #[no_mangle]
    pub fn rx(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };

//        self.enable_interrupts();
 //       self.enable_nvic();
        //self.set_rx_buffer();

        // RX ENABLE
        regs.RXEN.set(1);

        //regs.BCC.set(1);
        //regs.BCSTART.set(1);

        // Blocking Dummy Loop
        //while regs.READY.get() == 0 {}
        //regs.READY.set(0);

        // START RADIO

        //regs.START.set(1);


        //panic!("state: {}", regs.STATE.get());
        // Address Event
        //while /*regs.ADDRESS.get() {} == 0 && regs.PAYLOAD.get() == 0 ||*/ regs.BCMATCH.get() == 0 {}


        //panic!("Passed ADDRESS or PAYLOAD");

        // Address Event

        // Wait Until The Tranmission is Finished
        //while regs.END.get() == 0 {}

        //regs.DISABLE.set(1);

    }

    #[inline(never)]
    #[no_mangle]
    pub fn handle_interrupt(&self) {
        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };

        if regs.READY.get() == 1 {
            if regs.STATE.get() <= 4 {
                self.set_rx_buffer();
            }
            else {
                self.set_tx_buffer();
            }
            regs.READY.set(0);
            regs.END.set(0);
            regs.START.set(1);
        }

        if regs.PAYLOAD.get() == 1 {
            regs.PAYLOAD.set(0);
        }

        if regs.ADDRESS.get() == 1  {
            regs.ADDRESS.set(0);
            // panic!("ADDRESS");
        }

        if regs.END.get() == 1  {
            regs.END.set(0);
            regs.DISABLE.set(1);
            self.turnOnLeds();
            //unsafe {panic!("received message {:?}", rx_buf);}
        }

        // else
        // {
        //     panic!("STATE {:?}", regs.STATE.get());
        //     panic!("INTENSET {:?}\n", regs.INTENSET.get());
        // }
        nvic::clear_pending(NvicIdx::RADIO);
    }


    pub fn enable_interrupts(&self) {
        // INTENSET REG
        let regs: &mut Registers = unsafe { mem::transmute(self.regs) };
        regs.INTENSET.set(1 | 1 << 1 | 1 << 2 | 1 << 3);
    }

    // pub fn disable_interrupts(&self, interrupts: u32) {
    //     self.timer().intenclr.set(interrupts << 16);
    // }

    pub fn enable_nvic(&self) {
        nvic::enable(NvicIdx::RADIO);
    }

    pub fn disable_nvic(&self) {
        nvic::disable(NvicIdx::RADIO);
    }
    // WE NEED TO 2 additional rx and tx functions that shall be called upon an interrupt to comply
    // with the state machine :)
}

// Methods of RadioDummy Trait/Interface and are shared between Capsules and Chips
impl RadioDummy for Radio {

    // This Function is called once Tock is booted
    fn init(&self) {
        // panic!();
        self.config()
    }

    // This Function is called once a radio packet is to be sent
    fn send(&self) {
        self.tx();
    }

    // This Function is called once a radio packet is to be sent
    fn receive(&self) {
        self.rx();
    }

    // DUMMY FUNCTION  REMOVE LATER

    fn transmit(&self, dest: u16, tx_data: &'static mut [u8], tx_len: u8) -> ReturnCode {
        panic!("TRANSMIT WORKS WEEI");
    }


    fn dummy(&self) -> isize {
        1337
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn RADIO_Handler() {
    use kernel::common::Queue;
    nvic::disable(NvicIdx::RADIO);
    chip::INTERRUPT_QUEUE.as_mut().unwrap().enqueue(NvicIdx::RADIO);
}
