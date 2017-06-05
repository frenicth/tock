#include <stdio.h>
#include <stdbool.h>
#include <ble.h>
#include <timer.h>
#include <string.h>
#include "temperature.h"

static unsigned char t[1];
static unsigned char name[] = "TockOS";

static void callback(int temp,
    __attribute__ ((unused)) int not_used2,
    __attribute__ ((unused)) int arg2,
    __attribute__ ((unused)) void *ud) {
  t[0] = (unsigned char)temp;
}

int main(void)
{
  printf("starting BLE Temperature APP\r\n");

  temperature_init(callback, NULL);
  ble_adv_set_interval(10);

  for(;;) {
    temperature_measure();
    delay_ms(10);
    ble_adv_data(BLE_HS_ADV_TYPE_COMP_NAME, sizeof(name) - 1, name);
    ble_adv_data(BLE_HS_ADV_TYPE_MFG_DATA, 1, t);
    ble_adv_start();
    delay_ms(500);
    ble_adv_stop();
    ble_adv_clear_data();
  }

  return 0;
}
