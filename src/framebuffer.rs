use core::cell::UnsafeCell;

use crate::lcd::{LCD_HEIGHT, LCD_WIDTH};

pub const FB_WIDTH: usize = LCD_WIDTH;
pub const FB_HEIGHT: usize = LCD_HEIGHT;

#[repr(align(4))]
pub struct Framebuffer(pub UnsafeCell<[u16; FB_WIDTH * FB_HEIGHT]>);

// Drawing and transfer run on one core with interrupts disabled.
unsafe impl Sync for Framebuffer {}

pub static FRAMEBUFFER: Framebuffer = Framebuffer(UnsafeCell::new([0; FB_WIDTH * FB_HEIGHT]));

// RGB565: 16 bits per pixel.
pub fn clear_framebuffer() {
    unsafe {
        let framebuffer = &mut *FRAMEBUFFER.0.get();
        framebuffer.fill(0u16);
    }
}

pub fn fill_framebuffer(image: &[u8]) {
    assert_eq!(image.len(), FB_WIDTH * FB_HEIGHT * 2);

    unsafe {
        core::ptr::copy_nonoverlapping(
            image.as_ptr(),
            FRAMEBUFFER.0.get().cast::<u8>(),
            image.len(),
        );
    }
}
