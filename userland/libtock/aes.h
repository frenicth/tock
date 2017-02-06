#pragma once

#include <tock.h>

#define DRIVER_AES 34
#define KEY        0
#define ENC        1
#define DEC        2 

#ifdef __cplusplus
extern "C" {
#endif

int aes_init(subscribe_cb callback, void *ud);
int aes_configure_key(const char* packet, unsigned char len);
int aes_encrypt(const char* packet, unsigned char len);
int aes_decrypt(const char* packet, unsigned char len);


#ifdef __cplusplus
}
#endif

