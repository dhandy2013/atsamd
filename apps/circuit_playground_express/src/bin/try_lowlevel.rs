//! Circuit Playground Express app: Try out low-level routines
#![no_std]
#![no_main]

extern crate panic_halt;

use circuit_playground_express as hal;
use core::fmt::Write; // needed for writeln!()
use cortex_m_rt::entry;
use cpe_apps::ChipResources;
use cpe_apps::lowlevel::{asm_add, RegisterBuf};
use hal::prelude::*; // needed for .set_high(), .set_low(), and more

#[entry]
fn main() -> ! {
    let mut chip = ChipResources::new();
    writeln!(chip.stdout, "\nTrying out low-level routines!").ok();

    writeln!(chip.stdout, "\nAbout to call asm_add...").ok();
    let a = 2;
    let b = 3;
    let c = unsafe { asm_add(a, b) };
    writeln!(chip.stdout, "{} + {} = {}", a, b, c).ok();

    let mut reg = RegisterBuf::new();
    reg.load();
    writeln!(chip.stdout, "r0  = 0x{:08x}", reg.r0).ok();
    writeln!(chip.stdout, "r1  = 0x{:08x}", reg.r1).ok();
    writeln!(chip.stdout, "r2  = 0x{:08x}", reg.r2).ok();
    writeln!(chip.stdout, "r3  = 0x{:08x}", reg.r3).ok();
    writeln!(chip.stdout, "r4  = 0x{:08x}", reg.r4).ok();
    writeln!(chip.stdout, "r5  = 0x{:08x}", reg.r5).ok();
    writeln!(chip.stdout, "r6  = 0x{:08x}", reg.r6).ok();
    writeln!(chip.stdout, "r7  = 0x{:08x}", reg.r7).ok();
    writeln!(chip.stdout, "r8  = 0x{:08x}", reg.r8).ok();
    writeln!(chip.stdout, "r9  = 0x{:08x}", reg.r9).ok();
    writeln!(chip.stdout, "r10 = 0x{:08x}", reg.r10).ok();
    writeln!(chip.stdout, "r11 = 0x{:08x}", reg.r11).ok();
    writeln!(chip.stdout, "r12 = 0x{:08x}", reg.r12).ok();
    writeln!(chip.stdout, "sp  = 0x{:08x}", reg.sp).ok();
    writeln!(chip.stdout, "lr  = 0x{:08x}", reg.lr).ok();
    writeln!(chip.stdout, "pc  = 0x{:08x}", reg.pc).ok();

    loop {
        chip.red_led.set_high().unwrap();
        chip.delay.delay_ms(1000u16);
        chip.red_led.set_low().unwrap();
        chip.delay.delay_ms(1000u16);
    }
}
