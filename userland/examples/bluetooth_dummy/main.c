#include <stdio.h>
#include <bluetooth.h>
#include <button.h>
#include <timer.h>
#include <led.h>

#define BUF_SIZE 16
static void callback(int not_used, int not_used2,
		__attribute__ ((unused)) int arg2,
		__attribute__ ((unused)) void *ud){
  printf("callback\n");
}
int main(void)
{
  int num_leds = led_count();
  printf("BLUETOOTH SAMPLE APP\n");
  char packet[BUF_SIZE];
  for (int i = 0; i < BUF_SIZE; i++) { packet[i] = 9; }
  bluetooth_subscribe(callback, NULL);
  /*while (1) {
    delay_ms(1000);
    int a = bluetooth_call(1);
    printf("init %d\n");
  
   // int send = bluetooth_send(0, packet, BUF_SIZE);
   // printf("send %d\n", send);
  }*/
  return 0;
}
