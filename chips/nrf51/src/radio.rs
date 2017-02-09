use chip;
use core::cell::Cell;
use core::mem;
use kernel::common::VolatileCell;
use kernel::hil::radio_nrf51dk::{RadioDummy, Client};
use kernel::returncode::ReturnCode;
use kernel::common::take_cell::TakeCell;
use nvic;
use peripheral_interrupts::NvicIdx;

use gpio;
use kernel::hil::gpio::Pin;
extern crate capsules;
use self::capsules::led::LED;
use peripheral_registers::{RADIO_REGS, RADIO_BASE};

use test;
use bitfields::*;

#[deny(no_mangle_const_items)]

static mut tx_buf: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
static mut rx_buf: [u8; 16] = [0x00; 16];

#[no_mangle]
pub struct Radio {
    regs: *mut RADIO_REGS,
    client: Cell<Option<&'static Client>>,
    // tx_buffer: TakeCell<'static, [u8]>,
    // rx_buffer: TakeCell<'static, [u8]>,
}

pub static mut RADIO: Radio = Radio::new();


impl Radio {
    #[inline(never)]
    #[no_mangle]
    pub const fn new() -> Radio {
        Radio {
            regs: RADIO_BASE as *mut RADIO_REGS,
            client: Cell::new(None),
            // tx_buffer: TakeCell::empty(),
            // rx_buffer : TakeCell::empty(),
        }
    }
    pub fn set_client<C: Client>(&self, client: &'static C) {
        self.client.set(Some(client));
    }


    pub fn turnOnLeds(&self) {

        unsafe { let led0 = &gpio::PORT[21];
            led0.make_output();
            led0.toggle();
        }
    }


    pub fn config(&self) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        
        self.radio_on();

        self.set_txpower(0x04);

        self.set_channel_freq(0x07);
        
        self.set_data_white_iv(0x07);

        self.set_channel_rate(0x03);

        // according BLE standard
        // Use logical address 0 (prefix0 + base0) = 0x8E89BED6 when transmitting and receiving
        // CHECK THIS ONE
        regs.PREFIX0.set(0x8e);
        regs.BASE0.set(0x89bed500);

        self.set_tx_address(0x00);
        self.set_rx_address(0x01);
        regs.RXMATCH.set(0x00);

        // args not used atm
        self.set_channel_rate(0x00);
        // argument not t used ATM
        self.set_packet_config(0x00);
        self.set_crc_config(0x00);

        // Buffer configuration
        self.set_tx_buffer();

        self.enable_interrupts();
        self.enable_nvic();
    }

    pub fn set_crc_config(&self, val: u32) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        // CRC Config
        regs.CRCCNF.set(0x00);               // 3 bytes CRC
        regs.CRCINIT.set(0x00555555);        // INIT CRC Value
        regs.CRCPOLY.set(0x0000065B);        // POLYNOMIAL
    }
    
    
    // Packet configuration
    pub fn set_packet_config(&self, val: u32) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        
        regs.PCNF0.set(
            (PACKET0_S1_SIZE << RADIO_PCNF0_S1LEN_Pos) |
            (PACKET0_S0_SIZE << RADIO_PCNF0_S0LEN_Pos) |
            (PACKET_LENGTH_FIELD_SIZE << RADIO_PCNF0_LFLEN_Pos)
            );

        regs.PCNF1.set(
            (RADIO_PCNF1_WHITEEN_Disabled << RADIO_PCNF1_WHITEEN_Pos) |
            (RADIO_PCNF1_ENDIAN_Big       << RADIO_PCNF1_ENDIAN_Pos)  |
            (PACKET_BASE_ADDRESS_LENGTH   << RADIO_PCNF1_BALEN_Pos)   |
            (PACKET_STATIC_LENGTH         << RADIO_PCNF1_STATLEN_Pos) |
            (PACKET_PAYLOAD_MAXSIZE       << RADIO_PCNF1_MAXLEN_Pos)
            );
    }

    pub fn set_rx_address(&self, val: u32) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        regs.RXADDRESSES.set(0x01);
    }

    pub fn set_tx_address(&self, val: u32) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        regs.TXADDRESS.set(0x00);
    }
    
    pub fn set_channel_rate(&self, val: u32) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        // set channel rate,  3 - BLE 1MBIT/s
        regs.MODE.set(3);
    }
    pub fn set_data_white_iv(&self, val: u32) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        // DATAIV
        regs.DATAWHITEIV.set(7);
    }
    
    pub fn set_channel_freq(&self, val: u32) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        regs.FREQEUNCY.set(0x07);
    }
    
    pub fn radio_on(&self) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        // reset and enable power
        regs.POWER.set(0);
        regs.POWER.set(1);
    }
    
    pub fn set_txpower(&self, val: u32) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        // set tx power +4 dBm
        regs.TXPOWER.set(0x04);
    }
    
    pub fn set_tx_buffer(&self) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        unsafe {
            regs.PACKETPTR.set( (&tx_buf as *const u8) as u32);
        }
    }

    pub fn set_rx_buffer(&self) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        unsafe {
            regs.PACKETPTR.set( (&rx_buf as *const u8) as u32);
        }
    }

    #[inline(never)]
    #[no_mangle]
    pub fn tx(&self, dest: u16, tx_data: &'static mut [u8], tx_len: u8) {

        panic!("transmit {:?}\n", tx_data);
        
        for (i, c) in tx_data.as_ref()[0..16].iter().enumerate() {
            unsafe { tx_buf[i] = *c; }
        }

        self.set_tx_buffer();

        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };

        regs.READY.set(0);

        // TX ENABLE
        regs.TXEN.set(1);

        // // Blocking Dummy Loop
        // while regs.READY.get() == 0 {}
        //
        // //
        // regs.READY.set(0);
        // regs.END.set(0);
        // //
        // // // START RADIO
        // regs.START.set(1);
        // //
        // // // Address Event
        // while regs.PAYLOAD.get() == 0 {}
        // //
        // // // Wait Until The Tranmission is Finished
        // while regs.END.get() == 0 {}
        // //
        // regs.DISABLE.set(1);
        // panic!("WZUP");
    }

    #[inline(never)]
    #[no_mangle]
    pub fn rx(&self) {
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };

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
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };

        unsafe {self.client.get().map(|client|{client.receive_done(&mut rx_buf, 0)});}
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
            if regs.STATE.get() <= 4 { 
                unsafe {self.client.get().map(|client|{client.receive_done(&mut rx_buf, 0)});}
            }
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
        let regs: &mut RADIO_REGS = unsafe { mem::transmute(self.regs) };
        // 15 i.e set 4 LSB
        regs.INTENSET.set(1 | 1 << 1 | 1 << 2 | 1 << 3);
    }

    pub fn disable_interrupts(&self) {
        panic!("NOT IMPLEMENTED YET");
    }

    pub fn enable_nvic(&self) {
        nvic::enable(NvicIdx::RADIO);
    }

    pub fn disable_nvic(&self) {
        nvic::disable(NvicIdx::RADIO);
    }

}
// Methods of RadioDummy Trait/Interface and are shared between Capsules and Chips
impl RadioDummy for Radio {

    // This Function is called once Tock is booted
    fn init(&self) {
        self.config()
    }

    // This Function is called once a radio packet is to be sent
    fn send(&self) {
        unsafe { 
            self.tx(0, &mut tx_buf, 0); 
        }
        // panic!("NOT USED ATM");
    }

    // This Function is called once a radio packet is to be sent
    fn receive(&self) {
        self.rx();
    }

    #[inline(never)]
    #[no_mangle]
    fn transmit(&self, dest: u16, tx_data: &'static mut [u8], tx_len: u8) -> ReturnCode {
        
        panic!("transmit {:?}\n", tx_data);
        self.tx(dest, tx_data, tx_len);
        ReturnCode::SUCCESS
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
