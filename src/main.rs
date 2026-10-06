#![no_std]
#![no_main]

use core::arch::global_asm;

mod backlight;
mod framebuffer;
mod gpio;
mod lcd;
mod piezo;
mod registers;
mod timer;

global_asm!(include_str!("../main.s"));

const AUDIO: &[u8] = include_bytes!("../assets/bad_apple.raw");
// const IMAGE: &[u8] = include_bytes!("../assets/image.raw");
const VIDEO: &[u8] = include_bytes!("../assets/video.raw");

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    piezo::init();
    let mut lcd = lcd::init();
    framebuffer::clear_framebuffer();
    backlight::on();

    for frame in VIDEO.chunks_exact(320 * 240 * 2) {
        framebuffer::fill_framebuffer(frame);
        unsafe {
            lcd.update(&*framebuffer::FRAMEBUFFER.0.get());
        }
        timer::delay_ms(50);
    }

    loop {
        piezo::play_pcm_u8(AUDIO);
        backlight::off();
        timer::delay_ms(1_000);
    }
}
