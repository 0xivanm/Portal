#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("../main.s"));

const GPIOL_OUTPUT_VAL: *mut u32 = 0x6000d12c as *mut u32;

const DEV_INIT1: *mut u32 = 0x70000010 as *mut u32;
const DEV_EN: *mut u32 = 0x6000600c as *mut u32;
const PWM0_CTRL: *mut u32 = 0x7000a000 as *mut u32;

const DEV_PWM: u32 = 0x00020000;

fn gpio_set(output_reg: *mut u32, bits: u32) {
    let addr = output_reg as usize + 0x800;
    let val = (bits << 8) | bits;
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, val);
    }
}

fn gpio_clear(output_reg: *mut u32, bits: u32) {
    let addr = output_reg as usize + 0x800;
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

fn piezo_hw_init() {
    unsafe {
        let old = core::ptr::read_volatile(DEV_INIT1);
        core::ptr::write_volatile(DEV_INIT1, old & !0xc as u32);

        let old = core::ptr::read_volatile(DEV_EN);
        core::ptr::write_volatile(DEV_EN, old | DEV_PWM);
    }
}

fn piezo_hw_tick(form_and_period: u32) {
    unsafe {
        core::ptr::write_volatile(PWM0_CTRL, 0x80000000 | form_and_period);
    }
}

fn piezo_play(inv_freq: u16, form: u8) {
    let form_and_period = (form as u32) << 16 | (inv_freq as u32);
    piezo_hw_tick(form_and_period);
}

fn piezo_stop() {
    unsafe {
        core::ptr::write_volatile(PWM0_CTRL, 0x0);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    piezo_hw_init();

    loop {
        backlight_on();

        piezo_play(40, 0x80);
        delay(5_000_000);
        piezo_stop();

        backlight_off();
        delay(5_000_000);
    }
}