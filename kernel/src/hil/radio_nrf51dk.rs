use returncode::ReturnCode;

// Defines the Interface between Capsules and Chips
pub trait RadioDummy {
    fn init(&self);

    fn send(&self);
    fn receive(&self);
    fn transmit(&self, dest: u16, tx_data: &'static mut [u8], tx_len: u8) -> ReturnCode;
    // ADD MORE LATER

    fn dummy(&self) -> isize;
}
