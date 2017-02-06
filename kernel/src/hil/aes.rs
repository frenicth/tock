use returncode::ReturnCode;

pub trait AESDriver {
    fn init(&self, key: &'static mut [u8]) -> ReturnCode;
    fn encrypt(&self, plaintext: &'static mut [u8]) -> &'static mut [u8];
    fn decrypt(&self, ciphertext: &'static mut [u8]) -> &'static mut [u8];
}

pub trait Client {
    fn encrypt_done(&self) -> ReturnCode;
    fn decrypt_done(&self) -> ReturnCode;
}
