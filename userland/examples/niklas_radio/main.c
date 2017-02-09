#include <stdio.h>
#include <niklas.h>
#include <timer.h>

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
  
  int ret = subscribe_rx(callback, NULL);
  printf("subscribe %d\n", ret);

  for (int i = 0; i < 5; i++) {
    int send = tx_data(packet, BUF_SIZE);
    printf("send %d\n", send);
    delay_ms(1000);
  }
  return 0;
}
