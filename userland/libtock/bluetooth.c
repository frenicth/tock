#include "tock.h"
#include "bluetooth.h"

int bluetooth_call(int device) {
  return command(BLUETOOTH, 1, device);
}

int bluetooth_subscribe(subscribe_cb callback, void *ud) {
  return subscribe(BLUETOOTH, 0, callback, ud);
}


int bluetooth_send(unsigned short addr, const char* packet, unsigned char len) {
  allow(BLUETOOTH, 0, (void*)packet, SIZE);
}
