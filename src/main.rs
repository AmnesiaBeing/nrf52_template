#![no_main]
#![no_std]

// #[allow(unused_imports)]
use panic_semihosting as _;

use cortex_m_semihosting::syscall;
use nrf52832_hal as hal;

#[rtic::app(device = crate::hal::pac, peripherals = true)]
const APP: () = {
    #[init]
    fn init(_cx: init::Context) {
        // File descriptor (on the host)
        const STDOUT: usize = 1; // NOTE the host stdout may not always be fd 1
        static MSG: &'static [u8] = b"Hello, world!\n";

        // Signature: fn write(fd: usize, ptr: *const u8, len: usize) -> usize
        let _ = unsafe { syscall!(WRITE, STDOUT, MSG.as_ptr(), MSG.len()) };
    }
};
