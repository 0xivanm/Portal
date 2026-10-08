use core::arch::asm;

use crate::platform::registers::{CLOCK_SOURCE, CPU_CTRL, DEV_INIT2, DEV_TIMING1, PLL_CONTROL, PLL_STATUS};

const INIT_PLL: u32 = 0x4000_0000;

// Startup only: CPU core, interrupts masked, COP asleep.
#[unsafe(link_section = ".icode.clock")]
#[inline(never)]
pub fn set_30mhz() {
    unsafe {
        DEV_INIT2.modify(|value| value | INIT_PLL);
        PLL_CONTROL.modify(|value| value | 0x8800_0000);

        CPU_CTRL.write(0x4800_0003);
        asm!("nop", options(nomem, nostack, preserves_flags));
        CLOCK_SOURCE.write(0x2000_2222);
        DEV_TIMING1.write(0x0000_0303);
        CPU_CTRL.write(0x4800_001F);
        asm!("nop", options(nomem, nostack, preserves_flags));

        // PP5022: 24 MHz * 5 / 1 / 4 = 30 MHz.
        PLL_CONTROL.write(0x8A22_0501);
        while PLL_STATUS.read() & 0x8000_0000 == 0 {
            core::hint::spin_loop();
        }

        CPU_CTRL.write(0x4800_0003);
        asm!("nop", options(nomem, nostack, preserves_flags));
        DEV_TIMING1.write(0x0000_0303);
        CLOCK_SOURCE.write(0x2000_7777);
        CPU_CTRL.write(0x4800_001F);
        asm!("nop", options(nomem, nostack, preserves_flags));
    }
}
