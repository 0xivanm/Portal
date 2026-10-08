use core::arch::global_asm;

pub mod clock;
pub mod display;
pub mod gpio;
pub mod registers;
pub mod timer;

global_asm!(include_str!("start.s"));
