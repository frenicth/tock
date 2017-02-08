use returncode::ReturnCode;

pub trait AESDriver {
    fn init(&self) -> ReturnCode;
    fn set_key(&self, key: &'static mut [u8]) -> &'static mut [u8];
    fn encrypt(&self, plaintext: &'static mut [u8]) -> &'static mut [u8];
    fn decrypt(&self, ciphertext: &'static mut [u8]) -> &'static mut [u8];
}

pub trait Client {
    fn encrypt_done(&self) -> ReturnCode;
    fn decrypt_done(&self) -> ReturnCode;
    fn set_key_done(&self) -> ReturnCode;
}
