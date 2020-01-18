//! David's First Rust App on the Circuit Playground Express board
#![no_std]
#![no_main]

extern crate circuit_playground_express as hal;
extern crate cortex_m_rt;
extern crate panic_halt;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::pac::{CorePeripherals, Peripherals};

use cortex_m_rt::entry;

// PA17 == samd21g18a pin 26 == Circuit Playground Express signal D13
type RedLED = hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>;

fn dit(delay: &mut Delay, red_led: &mut RedLED) {
    red_led.set_high().unwrap();
    delay.delay_ms(250u8);
    red_led.set_low().unwrap();
    delay.delay_ms(250u8);
}

fn dah(delay: &mut Delay, red_led: &mut RedLED) {
    red_led.set_high().unwrap();
    delay.delay_ms(750u16);
    red_led.set_low().unwrap();
    delay.delay_ms(250u16);
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        dit(&mut delay, &mut red_led);
        dit(&mut delay, &mut red_led);
        dit(&mut delay, &mut red_led);
        dah(&mut delay, &mut red_led);
        delay.delay_ms(2000u16);
    }
}
