#include <stdio.h>
#include <aes.h>
#include <string.h>
#include <led.h>
#include <timer.h>
#include <rng.h>

static int cnt = 0;
static unsigned char data[1024];
/* INITIAL COUNTER */
static unsigned char ctr[] = {0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff};


static void callback(int cb,
    __attribute__ ((unused)) int len,
    __attribute__ ((unused)) int arg2,
    __attribute__ ((unused)) void *ud) {

  if ( cb == 0 ) {
    led_on(3);
    if(aes128_encrypt_ctr(data, 64, ctr, sizeof(ctr)) < 0) {
      printf("encrypt error\r\n");
    }

  }

  if ( cb == 1 )
  {
    led_off(3);
    cnt += 1;
    delay_ms(500);
    // take 10 measurements
    if(cnt < 10) {
      led_on(3);
      if(aes128_encrypt_ctr(data, 1024, ctr, sizeof(ctr)) < 0) {
        printf("encrypt error\r\n");
      }
    }
  }
}

int main(void)
{
  /* SET KEY */
  unsigned char key[] = {0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c};

  delay_ms(2000);
  rng_sync(data, 1024, 1024);

  printf("RANDOM BYTES\r\n");
  for(int i = 0; i < 1024; i++) {
    printf("%02x ", data[i]);
  }
  printf("\r\n");

  aes128_init(callback, NULL);

  int config = aes128_configure_key(key, sizeof(key));
  if(config < 0) {
    printf("set key error %d\r\n", config);
  }
  return 0;
}
