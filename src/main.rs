#![no_std]
#![no_main]

use core::arch::global_asm;

mod backlight;
mod gpio;
mod piezo;
mod registers;
mod timer;

global_asm!(include_str!("../main.s"));

const AUDIO: &[u8] = include_bytes!("../assets/bad_apple.raw");

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    piezo::init();

    loop {
        backlight::on();
        piezo::play_pcm_u8(AUDIO);
        backlight::off();
        timer::delay_ms(1_000);
    }
}
