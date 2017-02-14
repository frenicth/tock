#include <stdio.h>
#include <niklas.h>
#include <timer.h>


#define BUF_SIZE 16 


char packet[BUF_SIZE];

static void callback(int type, int not_used2,
    __attribute__ ((unused)) int arg2,
    __attribute__ ((unused)) void *ud){
  printf("CALLBACK\r\n");
  printf("data: \r\n");
  for(int i = 0; i < BUF_SIZE; i++) {
    printf(" 0x%x\r\n ", packet[i]);
    //printf(" %d\r\n ", packet[i]);
  }
}
int main(void)
{
  int halfSEc = 0;
  printf("demo app\r\n");
  for (int j = 0; j < BUF_SIZE; j++){
    packet[j] = 77;
  }
  for(;;) {
  	int ret = subscribe_rx(callback, NULL);
  	printf("subscribe %d\n", ret);
  
  	delay_ms(100);
  	rx_data(packet,BUF_SIZE);
	delay_ms(2000);
  }
  return 0;
}
