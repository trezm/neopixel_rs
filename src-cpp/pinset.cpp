#include "pinset.hpp"

STM32_Pin_Info* PIN_MAP2 = HAL_Pin_Map(); // Pointer required for highest access speed
#define pinLO(_pin) (PIN_MAP2[_pin].gpio_peripheral->BSRRH = PIN_MAP2[_pin].gpio_pin)
#define pinHI(_pin) (PIN_MAP2[_pin].gpio_peripheral->BSRRL = PIN_MAP2[_pin].gpio_pin)
#define pinSet(_pin, _hilo) (_hilo ? pinHI(_pin) : pinLO(_pin))

void neopixelFastSetPin(uint8_t pinNumber, bool on) {
  pin_t pin;
  switch (pinNumber) {
    case 1:
      pin = D1;
      break;
    case 2:
      pin = D2;
      break;
    case 3:
      pin = D3;
      break;
    case 4:
      pin = D4;
      break;
    case 5:
      pin = D5;
      break;
    case 6:
      pin = D6;
      break;
    case 7:
      pin = D7;
      break;
    default:
      pin = D2;
  }

  // pinSetFast(PIN_MAP2[_pin]
  // pinSet(pin, on ? HIGH : LOW);
  if (on) {
    pinSetFast(pin);
  } else {
    pinResetFast(pin);
  }
}
