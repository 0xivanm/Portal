use core::ptr::{read_volatile, write_volatile};

pub struct Register32 {
    address: usize,
}

impl Register32 {
    pub const fn new(address: usize) -> Self {
        Self { address }
    }

    pub const fn address(&self) -> usize {
        self.address
    }

    #[inline(always)]
    pub unsafe fn read(&self) -> u32 {
        unsafe { read_volatile(self.address as *const u32) }
    }

    #[inline(always)]
    pub unsafe fn write(&self, value: u32) {
        unsafe { write_volatile(self.address as *mut u32, value) }
    }

    #[inline(always)]
    pub unsafe fn modify(&self, change: impl FnOnce(u32) -> u32) {
        unsafe { self.write(change(self.read())) }
    }
}

pub const DEV_INIT1: Register32 = Register32::new(0x7000_0010);
pub const DEV_EN: Register32 = Register32::new(0x6000_600C);
pub const PWM0_CTRL: Register32 = Register32::new(0x7000_A000);

// read-only free-running microsecond counter
pub const USEC_TIMER: Register32 = Register32::new(0x6000_5010);

pub const GPIOL_OUTPUT_VAL: Register32 = Register32::new(0x6000_D12C);
