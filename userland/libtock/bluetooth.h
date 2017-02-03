#pragma once

#include "tock.h"

#define BLUETOOTH 33
#define SIZE 16


int bluetooth_call(int device);

int bluetooth_subscribe(subscribe_cb callback, void *ud);


int bluetooth_send(unsigned short addr, const char* packet, unsigned char len);
