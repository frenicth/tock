#include <stdio.h>
#include <niklas.h>
#include <timer.h>

#define RECEIVER

#define BUF_SIZE 16
static void callback(int type, int not_used2,
		__attribute__ ((unused)) int arg2,
		__attribute__ ((unused)) void *ud){
  
  if (type == 12 ) printf("callback rx\n");
  else if (type == 13) printf("callback tx\n");
}
int main(void)
{
  printf("demo app\n");
  char packet[BUF_SIZE];
  for (int j = 0; j < BUF_SIZE; j++){
	packet[j] = 77;
  } 
#ifdef RECEIVER 
  int ret = subscribe_rx(callback, NULL);
  printf("subscribe %d\n", ret);
  for(;;){
    //printf("in receive mode\n");
   rx_data(packet,BUF_SIZE);
   delay_ms(150);
  }
#else
  int ret = subscribe_tx(callback, NULL);
  for (;;) {
    int send = tx_data(packet, BUF_SIZE);
    printf("send %d\n", send);
    delay_ms(1000);
  }
#endif
  return 0;
}
