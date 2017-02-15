use returncode::ReturnCode;

// Defines the Interface between Capsules and Chips
pub trait RadioDummy {
    fn init(&self);

    fn send(&self);
    fn receive(&self);
    fn transmit(&self, dest: u16, tx_data: &'static mut [u8], tx_len: u8) -> ReturnCode;
    // ADD MORE LATER

    fn dummy(&self) -> isize;
    fn set_channel(&self, ch: usize);
}

pub trait Client {
    /// Called when a rx or tx is finished
    fn receive_done(&self, rx_data: &'static mut [u8], len: u8) -> ReturnCode;
    fn transmit_done(&self, tx_data: &'static mut [u8], len: u8) -> ReturnCode;
}
