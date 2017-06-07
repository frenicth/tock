#include "temperature.h"

struct temp_data {
  bool fired;
  int value;
};

static struct temp_data result = { .fired = false };


// Internal callback for faking synchronous reads
static void callback(int temp,
    __attribute__ ((unused)) int not_used2,
    __attribute__ ((unused)) int arg2,
    __attribute__ ((unused)) void *ud) {
   result.value = temp;
   result.fired = true;
}

int temperature_set_callback(subscribe_cb callback, __attribute__ ((unused)) void *ud) {
  return subscribe(DRIVER_TEMP, 0, callback, NULL);
}

int temperature_get() {
  return command(DRIVER_TEMP, 0, 0);
}


int temperature_measure(void) {
    int err;
    result.fired = false;

    err = temperature_set_callback(callback, (void*) &result);
    if (err < 0) return err;

    err = temperature_get();
    if (err < 0) return err;

    // Wait for the callback.
    yield_for(&result.fired);

    return result.value;
}
