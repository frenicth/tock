#pragma once

#include "tock.h"

#define BLUETOOTH 33

int bluetooth_call(int device);

int bluetooth_subscribe(subscribe_cb callback, void *ud);
