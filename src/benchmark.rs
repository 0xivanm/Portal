use core::sync::atomic::{Ordering, compiler_fence};

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

            compiler_fence(Ordering::SeqCst);
            let start = timer::micros();
            tests[index](core::hint::black_box(input));
            compiler_fence(Ordering::SeqCst);
            let elapsed = timer::micros().wrapping_sub(start);

            totals[index] += u64::from(elapsed);
        }
    }

    totals.map(|total| (total / u64::from(rounds)) as u32)
}

pub fn draw_bars(fb: &mut [u16; LCD_WIDTH * LCD_HEIGHT], times: [u32; 3]) {
    fb.fill(0);
    let longest = times.into_iter().max().unwrap_or(1).max(1);
    let colors = [0xF800, 0x07E0, 0x001F];

    // One scale for all bars; longer means slower. Longest fills the ruler.
    for y in 24..216 {
        fb[y * LCD_WIDTH + 19] = 0xFFFF;
        fb[y * LCD_WIDTH + 300] = 0x4208;
    }

    for (index, time) in times.into_iter().enumerate() {
        let width = ((u64::from(time) * 280) / u64::from(longest)) as usize;
        let y = 40 + index * 64;
        for row in y..y + 24 {
            fb[row * LCD_WIDTH + 20..row * LCD_WIDTH + 20 + width].fill(colors[index]);
        }
    }
}
