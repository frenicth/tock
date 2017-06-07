#include <stdio.h>
#include <stdbool.h>
#include <ble.h>
#include <rng.h>
#include <timer.h>
#include <string.h>
#include "temperature.h"

int main(void)
{
  printf("\rstarting BLE Temperature APP\r\n");
  unsigned char name[] = "TockOS";
  unsigned char buf[5];
  
  int err;

  for(;;) {
    err = temperature_measure();
    if(err >= 0) {
      buf[0] = (unsigned char)err;
    }
    else {
      printf("Negative temperature value discard it\r\n");
    }
    
    if(rng_sync(buf+1, 4, 4) < 0) {
      printf("rng error\r\n");
    }

    // start
    ble_adv_data(BLE_HS_ADV_TYPE_COMP_NAME, sizeof(name) - 1, name);
    ble_adv_data(BLE_HS_ADV_TYPE_MFG_DATA, sizeof(buf), buf);
    ble_adv_start();
    
    // sleep
    delay_ms(1000);
    
    // top
    ble_adv_stop();
    ble_adv_clear_data();

  }

  return 0;
}
