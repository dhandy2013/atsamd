//! Dump the entire 256kb contents of the SAMD21G18A firmware in hex format.
#![no_std]
#![no_main]

extern crate panic_halt;

use circuit_playground_express as hal;
use core::fmt::Write; // needed for write!(), writeln!()
use cortex_m_rt::entry;
use cpe_apps::ChipResources;
use hal::prelude::*; // needed for .set_high(), .set_low(), and more

/// Return the byte at the integer memory location.
/// I wrote this function because I need to read memory starting at address
/// zero, and Rust doesn't let you form slices starting at address zero.
/// (It refuses to iterate over them, stops iteration early.)
unsafe fn peek_byte(loc: usize) -> u8 {
    *(loc as *const u8)
}

#[entry]
fn main() -> ! {
    let mut chip = ChipResources::new();
    let start_loc: usize = 0;
    let size: usize = 256 * 1024;
    writeln!(
        chip.stdout,
        "\n# BEGIN HEXDUMP: {} bytes starting at 0x{:08x}",
        size, start_loc
    )
    .ok();

    for offset in 0..size {
        let byte = unsafe { peek_byte(start_loc + offset) };
        write!(chip.stdout, "{:02X}", byte).ok();
        if (offset & 0xf) == 0xf {
            chip.red_led.set_high().unwrap();
            write!(chip.stdout, "\n").ok();
            chip.red_led.set_low().unwrap();
        } else {
            write!(chip.stdout, " ").ok();
        }
    }
    if (size & 0xf) != 0 {
        writeln!(chip.stdout, "").ok();
    }

    writeln!(chip.stdout, "# END HEXDUMP").ok();
    loop {
        chip.red_led.set_high().unwrap();
        chip.delay.delay_ms(1000u16);
        chip.red_led.set_low().unwrap();
        chip.delay.delay_ms(1000u16);
    }
}
