#![no_std]
#![no_main]

use core::arch::global_asm;

mod backlight;
mod benchmark;
mod framebuffer;
mod gpio;
mod lcd;
mod piezo;
mod registers;
mod timer;

global_asm!(include_str!("../main.s"));
global_asm!(include_str!("../lcd-transfer.s"), options(raw));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let mut lcd = lcd::init();
    backlight::on();

    unsafe {
        let fb = &mut *framebuffer::FRAMEBUFFER.0.get();
        fb.fill(0xFFFF);

        let times = benchmark::compare(
            &fb[..],
            12,
            lcd::prepare_transfer,
            [
                lcd::lcd_write_data,
                lcd::lcd_write_data_iram,
                lcd::lcd_write_data_asm,
            ],
        );

        benchmark::draw_bars(fb, times);
        lcd.update(fb);
    }

    loop {
        core::hint::spin_loop();
    }
}
