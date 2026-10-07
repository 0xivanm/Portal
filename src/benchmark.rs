use core::arch::asm;

use crate::lcd::{LCD_HEIGHT, LCD_WIDTH};
use crate::timer;

pub fn compare<T: ?Sized, const N: usize>(input: &T, rounds: u32, mut prepare: impl FnMut(), tests: [fn(&T); N]) -> [u32; N] {
    assert!(rounds != 0 && N != 0);
    let mut totals = [0u64; N];

    for test in tests {
        prepare();
        test(core::hint::black_box(input));
    }

    for round in 0..rounds {
        for offset in 0..N {
            let index = (round as usize + offset) % N;
            prepare();

            unsafe { asm!("", options(nostack, preserves_flags)) };
            let start = timer::micros();
            tests[index](core::hint::black_box(input));
            unsafe { asm!("", options(nostack, preserves_flags)) };
            let elapsed = timer::micros().wrapping_sub(start);

            totals[index] += u64::from(elapsed);
        }
    }

    totals.map(|total| (total / u64::from(rounds)) as u32)
}

pub fn draw_bars<const N: usize>(fb: &mut [u16; LCD_WIDTH * LCD_HEIGHT], times: [u32; N], colors: [u16; N]) {
    assert!(N != 0 && N <= LCD_HEIGHT - 48);
    fb.fill(0);

    let longest = times.into_iter().max().unwrap_or(1).max(1);
    let slot = (LCD_HEIGHT - 48) / N;
    let height = slot.min(24);

    // All bars share one scale: longer means slower.
    for y in 24..LCD_HEIGHT - 24 {
        fb[y * LCD_WIDTH + 19] = 0xFFFF;
        fb[y * LCD_WIDTH + LCD_WIDTH - 20] = 0x4208;
    }

    for (index, time) in times.into_iter().enumerate() {
        let width = (u64::from(time) * (LCD_WIDTH - 40) as u64 / u64::from(longest)) as usize;
        let y = 24 + index * slot + (slot - height) / 2;
        for row in y..y + height {
            fb[row * LCD_WIDTH + 20..row * LCD_WIDTH + 20 + width].fill(colors[index]);
        }
    }
}
