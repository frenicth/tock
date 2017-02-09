#include <stdio.h>
#include <aes.h>
#include <timer.h>

#define SIZE 16

static void callback(void* buffer, int buffer_len,
    __attribute__ ((unused)) int arg2,
    __attribute__ ((unused)) void *ud){
  printf("buffer %d\n", buffer);
  printf("buffer_len %d\n", buffer_len);
  printf("callback\n");
  printf("arg2 %d\n", arg2);
  printf("*ud %d\n", ud);
  printf("ud bool %d\n", (*(bool*)ud));
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
    int enc = aes_encrypt(plaintext, SIZE);
    printf("encrypt return %d\n", enc);
    
    /** delay_ms(1000); */
    /** int dec = aes_decrypt(plaintext, SIZE); */
    /** printf("decrypt return %d\n", dec); */
  }
  return 0;
}
