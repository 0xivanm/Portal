#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("../main.s"));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const GPIOL_OUTPUT_VAL: usize = 0x6000d12c;

fn gpio_set(output_reg: usize, bits: u32) {
    let addr = output_reg + 0x800;
    let val = (bits << 8) | bits;
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, val);
    }
}

fn gpio_clear(output_reg: usize, bits: u32) {
    let addr = output_reg + 0x800;
    let val = bits << 8;
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, val);
    }
}

fn backlight_on() {
    gpio_set(GPIOL_OUTPUT_VAL, 0x80);
}

fn backlight_off() {
    gpio_clear(GPIOL_OUTPUT_VAL, 0x80);
}

fn delay(mut count: u32) {
    while count != 0 {
        unsafe {
            core::arch::asm!("nop", options(nomem, nostack));
        }
        count -= 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    loop {
        backlight_on();
        delay(1_000_000);
        backlight_off();
        delay(1_000_000);
    }
}