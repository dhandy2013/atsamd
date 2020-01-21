//! David's UART stdout demo app for the Circuit Playground Express
#![no_std]
#![no_main]

extern crate circuit_playground_express as hal;
extern crate cortex_m_rt;
extern crate panic_halt;

use core::fmt::Write; // needed for writeln!()
use cortex_m_rt::entry;
use cpe_apps::ChipResources;
use hal::prelude::*; // needed for .set_high(), .set_low(), and more

#[entry]
fn main() -> ! {
    let mut chip = ChipResources::new();
    writeln!(chip.stdout, "\nHello from demo_stdout!").ok();
    let a = 2;
    let b = 3;
    writeln!(chip.stdout, "a + b = {}", a + b).ok();

    loop {
        chip.red_led.set_high().unwrap();
        chip.delay.delay_ms(1000u16);
        chip.red_led.set_low().unwrap();
        chip.delay.delay_ms(1000u16);
    }
}
