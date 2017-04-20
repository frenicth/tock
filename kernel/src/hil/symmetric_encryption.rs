//! Interfaces for accessing encryption and decryption of symmetric ciphers
//! (only AES-128-ctr for suppported nrf51dk "at the moment")
//!
//! The interface is supposed to work for hardware supported crypto but
//! should work for software implemented crypto as well.
//!
//! State Machine:
//!  1. init()
//!  2. set_key()
//!  3. aes128_crypt_ctr()   - can be used arbitary number of times
//!

use returncode::ReturnCode;

pub trait SymmetricEncryptionDriver {
    /// Initialization of the chip register
    fn init(&self);

    /// Configure encryption/decryption key
    /// assumes that key size is 16, 24 or 32 bytes
    fn set_key(&self, key: &'static mut [u8], len: usize);

    /// encryption and decryption for aes in counter mode
    /// because only the encryption-mode of the cipher only one method is needed
    /// other chips perhaps only ignore "init_ctr" and assume all is performed in HW
    fn aes128_crypt_ctr(&self, data: &'static mut [u8], init_ctr: &'static mut [u8], len: usize);
}

pub trait Client {
    /// send back to result of the encryption/decryption to the capsule
    /// this should be hardware independent if the cryptostate is used for all
    /// implementations
    fn crypt_done(&self, data: &'static mut [u8], dmy: &'static mut [u8], len: usize) -> ReturnCode;

    /// once the key has been configure trigger call-back to indicate to the capsule
    /// that now it's possible to begin to encrypt and decrypt data
    fn set_key_done(&self, key: &'static mut [u8]) -> ReturnCode;
}
