#![no_std]
#![no_main]

mod drivers;
mod graphics;
mod platform;
mod debug;

use crate::drivers::{backlight, lcd};
use crate::graphics::framebuffer;
use crate::platform::{clock, timer};

const FPS: u32 = 18;
const FRAME_BYTES: usize = framebuffer::FB_WIDTH * framebuffer::FB_HEIGHT * 2;

#[repr(align(4))]
struct Video([u8; include_bytes!("../assets/video.raw").len()]);

static VIDEO: Video = Video(*include_bytes!("../assets/video.raw"));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    clock::set_frequency(clock::CpuFrequency::Mhz30);
    let mut lcd = lcd::init();
    let framebuffer = unsafe { framebuffer::take() };
    backlight::on();

    assert!(!VIDEO.0.is_empty());
    assert_eq!(VIDEO.0.len() % FRAME_BYTES, 0);

    loop {
        let mut deadline = timer::micros();
        for frame in VIDEO.0.chunks_exact(FRAME_BYTES) {
            framebuffer.fill(frame);
            lcd.update(framebuffer.pixels());

            // Copying and transfer count toward the frame interval.
            deadline = deadline.wrapping_add(1_000_000 / FPS);
            timer::wait_until(deadline);
        }
    }
}
