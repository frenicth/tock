use hil;

/// Interface for banks of asynchronous GPIO pins. GPIO pins are asynchronous
/// when there is an asynchronous interface used to control them. The most
/// common example is when using a GPIO extender on an I2C or SPI bus. With
/// asynchronous GPIO functions, every config action results in an eventual
/// callback function that indicates that the configuration has finished
/// (unless the initial function call returns an error code, then no callback
/// will be generated).
///
/// Asynchronous GPIO pins are grouped into ports because it is assumed that
/// the remote entity that is controlling the pins can control multiple pins.
/// Typically, a port will be provided by a particular driver.
///
/// The API for the GPIOAsyncPort mirrors the synchronous GPIO interface.
pub trait GPIOAsyncPort {
    /// Try to disable a GPIO pin. This cannot be supported for all devices.
    fn disable(&self, pin: usize) -> isize;

    /// Configure a pin as an ouput GPIO.
    fn enable_output(&self, pin: usize) -> isize;

    /// Configure a pin as an input GPIO. Not all InputMode settings may
    /// be supported by a given device.
    fn enable_input(&self, pin: usize, mode: hil::gpio::InputMode) -> isize;

    /// Get the state (0 or 1) of an input pin. The value will be returned
    /// via a callback.
    fn read(&self, pin: usize) -> isize;

    /// Toggle an output GPIO pin.
    fn toggle(&self, pin: usize) -> isize;

    /// Assert a GPIO pin high.
    fn set(&self, pin: usize) -> isize;

    /// Clear a GPIO pin low.
    fn clear(&self, pin: usize) -> isize;

    /// Setup an interrupt on a GPIO input pin. The identifier should be
    /// the port number and will be returned when the interrupt callback
    /// fires.
    fn enable_interrupt(&self, pin: usize, mode: hil::gpio::InterruptMode, identifier: usize) -> isize;

    /// Disable an interrupt on a GPIO input pin.
    fn disable_interrupt(&self, pin: usize) -> isize;
}

/// The gpio_async Client interface is used to both receive callbacks
/// when a configuration command finishes and to handle interrupt events
/// from pins with interrupts enabled.
pub trait Client {
    /// Called when an interrupt occurs. The identifier is split between
    /// the port and pin numbers that triggered the interrupt. The lowest
    /// eight bits are the port number, and the next lowest eight bits
    /// are the pin in that port that triggered the interrupt.
    fn fired(&self, identifier: usize);

    /// Done is called when a configuration command finishes.
    fn done(&self, value: usize);
}
