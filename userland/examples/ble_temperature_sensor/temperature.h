#pragma once

#include <tock.h>

#ifdef __cplusplus
extern "C" {
#endif

#define DRIVER_TEMP 36

int temperature_set_callback(subscribe_cb callback, void *ud);
int temperature_get();
int temperature_measure(void);

#ifdef __cplusplus
}
#endif

