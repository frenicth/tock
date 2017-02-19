#include <stdio.h>
#include <aes.h>
#include <timer.h>

#define SIZE 16

static char plaintext[SIZE];

static void callback(int cb, int len,
    __attribute__ ((unused)) int arg2,
    __attribute__ ((unused)) void *ud){

  if ( cb == 0 ) {
    printf("KEY IS CONFIGURED\r\n");
  }

  if ( cb == 1 ) 
  {
    printf("CIPHERTEXT: \r\n");
    for (int i = 0; i < SIZE; i++) {
      printf("%d ", plaintext[i]);
    }
    printf("\r\n");
  }


  if ( cb == 2 ) 
  {
    printf("PLAINTEXT: \r\n");
    for (int i = 0; i < SIZE; i++) {
      printf("%d ", plaintext[i]);
    }
    printf("\r\n");
  }


}

int main(void)
{
  char key[SIZE];

  for (int i = 0; i < SIZE; i++) {
    plaintext[i] = 9;
    key[i] = i;
  }

  // SUBSCRIBE
  aes_ccm_init(callback, NULL);
  int config = aes_ccm_configure_key(key, SIZE);
  
  for (int i = 0; i < 10; i++) {
    // ALLOW + COMMAND
    delay_ms(500);
    if (aes_ccm_encrypt(plaintext, SIZE) < 0) {
      printf("encrypt error\r\n");
    }
    delay_ms(500);
    if (aes_ccm_decrypt(plaintext, SIZE) < 0) {
      printf("decrypt error\r\n");
    }
  }
  return 0;
}
