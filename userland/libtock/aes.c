#include "aes.h"


struct aes_data {
  bool fired;
  int received;
};

static struct aes_data result = { .fired = false, .received = 0};


static void aes_cb(int cb,
    __attribute__ ((unused)) int len,
    __attribute__ ((unused)) int arg2,
    __attribute__ ((unused)) void *ud) {
  result.fired = true;
  result.received = cb;
}


int aes128_set_callback(subscribe_cb callback, void *ud) {
  return subscribe(AES_DRIVER, 0, callback, ud);
}

int aes128_configure_key(const unsigned char* key, unsigned char len) {
  int err = allow(AES_DRIVER, AES_KEY, (void*)key, len);
  if (err < 0)  {
    return err;
  }
  return command(AES_DRIVER, AES_KEY, 0);
}

int aes128_encrypt_ctr(unsigned const char* buf, unsigned char buf_len, unsigned const char* ctr, unsigned char ctr_len) {
  int err;
  
  err = aes128_set_callback(aes_cb, NULL);
  if (err < 0) return err;

  err = allow(AES_DRIVER, AES_DATA, (void*)buf, buf_len);
  if (err < 0) return err;
  
  result.fired = false;
  err = allow(AES_DRIVER, AES_CTR, (void*)ctr, ctr_len);
  if (err < 0) return err;
  
  return command(AES_DRIVER, AES_ENC, 0);
  yield_for(&result.fired);
  result.received;
}

int aes128_decrypt_ctr(const unsigned char* buf, unsigned char buf_len, const unsigned char* ctr, unsigned char ctr_len) {
  int err = aes_set_callback(aes_cb, NULL);
  if (err < 0) return err;

  err = allow(AES_DRIVER, AES_DATA, (void*)buf, buf_len);
  if (err < 0) return err;
  
  err = allow(AES_DRIVER, AES_CTR, (void*)ctr, ctr_len);
  if (err < 0) return err;
  
  result.fired = false;
  return command(AES_DRIVER, AES_DEC, 0);
  yield_for(&result.fired);
  return result.received;
}
