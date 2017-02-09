#include "aes.h"

int aes_init(subscribe_cb callback, void *ud) {
  char data[10];
  return subscribe(DRIVER_AES, 0, callback, (void*)data);
}

int aes_configure_key(const char* key, unsigned char len) {
  int err = allow(DRIVER_AES, KEY, (void*)key, len);
  if (err < 0)  {
    return err;
  }
  return command(DRIVER_AES, KEY, 0);
}

int aes_encrypt(const char* msg, unsigned char len) {
  int err = allow(DRIVER_AES, ENC, (void*)msg, len);
  if (err < 0)  {
    return err;
  }
  return command(DRIVER_AES, ENC, 0);
}

int aes_decrypt(const char* ciphertext, unsigned char len) {
  int err = allow(DRIVER_AES, DEC, (void*)ciphertext, len);
  if (err < 0)  {
    return err;
  }
  return command(DRIVER_AES, DEC, 0);
}
