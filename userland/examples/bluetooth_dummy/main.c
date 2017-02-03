#include <stdio.h>
#include <bluetooth.h>
#include <button.h>
#include <timer.h>
#include <led.h>

static void button_callback(int btn_num, int val, __attribute__ ((unused)) int arg2, __attribute__ ((unused)) void *ud)
{
  printf("received CALLBACK\n");
  /** if (val == 0) */
  /** { */
  /**   led_toggle(btn_num); */
  /** } */
}

int main(void)
{
  int num_leds = led_count();
  printf("BLUETOOTH SAMPLE APP\n");
  
  while (1)
  {
    delay_ms(1000);
    int a = bluetooth_call(1);
    printf("call %d\n", a);
  }
  // Enable interrupts on each button.
  /** bluetooth_subscribe(button_callback, NULL); */
  /** int count = button_count(); */
  /** for (int i = 0; i < count; i++) { */
  /**   button_enable_interrupt(i); */
  /** } */
  return 0;
}
