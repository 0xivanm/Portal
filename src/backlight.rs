use crate::gpio::{gpio_clear, gpio_set};
use crate::registers::GPIOL_OUTPUT_VAL;

const BACKLIGHT_BIT: u32 = 0x80;

pub fn on() {
    gpio_set(&GPIOL_OUTPUT_VAL, BACKLIGHT_BIT);
}

pub fn off() {
    gpio_clear(&GPIOL_OUTPUT_VAL, BACKLIGHT_BIT);
}
