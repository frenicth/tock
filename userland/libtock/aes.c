#include "aes.h"

int aes_init(subscribe_cb callback, void *ud) {
  return subscribe(DRIVER_AES, 0, callback, ud);
}

int aes_configure_key(const char* key, unsigned char len) {
  int err = allow(DRIVER_AES, KEY, (void*)key, len);
  if (err < 0)  {
    return err;
  }
  return command(DRIVER_AES, 0, 0);
}

int aes_encrypt(const char* msg, unsigned char len) {
  int err = allow(DRIVER_AES, ENC, (void*)msg, len);
  if (err < 0)  {
    return err;
  }
  return command(DRIVER_AES, 1, 0);
}

int aes_decrypt(const char* ciphertext, unsigned char len) {
  int err = allow(DRIVER_AES, DEC, (void*)ciphertext, len);
  if (err < 0)  {
    return err;
  }
  return command(DRIVER_AES, 2, 0);
}
