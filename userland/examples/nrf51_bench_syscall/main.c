#include <led.h>
#include <timer.h>

/*
 *  perform 10000 gpio/leds on/off to measure the average overhead of a syscall
 */

int main(void) {
  delay_ms(3000);
  bool toggle = false;
  led_on(3);
  for (int i = 0; i < 10000; i++) {
    if(toggle) {
      led_on(0);
    }
    else {
      led_off(0);
    }
    toggle = !toggle;
  }
  led_off(3);
  return 0;
}
