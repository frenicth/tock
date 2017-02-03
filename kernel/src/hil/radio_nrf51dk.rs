// use returncode::ReturnCode;

//! Defines the Interface between Capsules and Chips
pub trait RadioDummy {

    fn init(&self);

    fn send(&self);
    fn receive(&self);

    // ADD MORE LATER

    fn dummy(&self) -> isize;
}
