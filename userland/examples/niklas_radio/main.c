#include <stdio.h>
#include <niklas.h>
#include <timer.h>

//#define RECEIVER 
#define BUF_SIZE 16
static void callback(int not_used, int not_used2,
		__attribute__ ((unused)) int arg2,
		__attribute__ ((unused)) void *ud){
  printf("callback\n");
}
int main(void)
{
  printf("niklas app\n");
  char packet[BUF_SIZE];
  for (int j = 0; j < BUF_SIZE; j++){
	packet[j] = j;
  } 
#ifdef RECEIVER 
  int ret = subscribe_rx(callback, NULL);
  printf("subscribe %d\n", ret);
  for(;;){
    //printf("in receive mode\n");
   rx_data(packet,BUF_SIZE);
   delay_ms(1000);
  }
#else
  int send = tx_data(packet, BUF_SIZE);
  for (;;) {
    int send = tx_data(packet, BUF_SIZE);
    printf("send %d\n", send);
    delay_ms(1000);
  }
#endif
  return 0;
}
