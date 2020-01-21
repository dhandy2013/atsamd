//! Circuit Playground Express app: Blink morse code "V"
#![no_std]
#![no_main]

extern crate panic_halt;

use circuit_playground_express as hal;
use core::fmt::Write; // needed for writeln!()
use cortex_m_rt::entry;
use cpe_apps::ChipResources;
use hal::prelude::*; // needed for .set_high(), .set_low(), and more

fn dit(chip: &mut ChipResources) {
    chip.red_led.set_high().unwrap();
    chip.delay.delay_ms(250u8);
    chip.red_led.set_low().unwrap();
    chip.delay.delay_ms(250u8);
}

fn dah(chip: &mut ChipResources) {
    chip.red_led.set_high().unwrap();
    chip.delay.delay_ms(750u16);
    chip.red_led.set_low().unwrap();
    chip.delay.delay_ms(250u16);
}

#[entry]
fn main() -> ! {
    let mut chip = ChipResources::new();
    writeln!(chip.stdout, "\nSending the letter 'V' in Morse code").ok();

    loop {
        dit(&mut chip);
        dit(&mut chip);
        dit(&mut chip);
        dah(&mut chip);
        write!(chip.stdout, ".").ok();
        chip.delay.delay_ms(2000u16);
    }
}
