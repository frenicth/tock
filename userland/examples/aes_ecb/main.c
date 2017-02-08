#include <stdio.h>
#include <aes.h>
#include <timer.h>

#define SIZE 16

static void callback(int not_used, int not_used2,
    __attribute__ ((unused)) int arg2,
    __attribute__ ((unused)) void *ud){
  printf("callback\n");
}


int main(void)
{

  printf("AES ECB SAMPLE APP\n");

  char key[SIZE];
  char plaintext[SIZE];


  for (int i = 0; i < SIZE; i++) {
    plaintext[i] = 9;
    key[i] = 1;
  }

  // SUBSCRIBE 
  aes_init(callback, NULL);
  
  for (int i = 0; i < 1; i++) {
    // ALLOW + COMMAND
    int config = aes_configure_key(key, SIZE);
    printf("config_key return %d\n", config);
    delay_ms(1000);
  }
  /** int enc = aes_encrypt(plaintext, SIZE); */
  /** delay_ms(1000); */
  /** printf("encrypt return %d\n", enc); */
  return 0;
}
