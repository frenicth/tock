#include <stdio.h>
#include <stdbool.h>
#include <ble.h>
#include <rng.h>
#include <timer.h>
#include <string.h>
#include "temperature.h"
#include <aes.h>


int main(void)
{
  printf("\rstarting BLE Temperature APP\r\n");
  unsigned char name[] = "TockOS";
  unsigned char buf[5];
  unsigned char key[] = {0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c};

  unsigned char ctr[] = {0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff};

  int err;
  aes128_configure_key(key, sizeof(key));
  
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
    
    aes128_encrypt_ctr(buf, sizeof(buf), ctr, sizeof(ctr));

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
