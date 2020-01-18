//! David's App #2 on the Circuit Playground Express board:
//! Communicating over the UART
#![no_std]
#![no_main]

extern crate circuit_playground_express as hal;
extern crate cortex_m_rt;
extern crate panic_halt;

#[macro_use(block)]
extern crate nb;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::pac::{CorePeripherals, Peripherals};
use hal::pac::gclk::clkctrl::GEN_A;
use hal::pac::gclk::genctrl::SRC_A;
use hal::sercom::{PadPin, Sercom4Pad0, Sercom4Pad1, UART4};

use cortex_m_rt::entry;

static MESSAGE: &[u8] = b"\r\nThe rain in spain falls mainly in the plain!\r\n";

// PA17 == samd21g18a pin 26 == Circuit Playground Express signal D13
type RedLED = hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>;

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
    let mut delay = Delay::new(core.SYST, &mut clocks);

    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL48M, false);
    let gclk2 = clocks
        .get_gclk(GEN_A::GCLK2)
        .expect("Could not get clock 2");
    let uart_clk = clocks
        .sercom4_core(&gclk2)
        .expect("Could not configure sercom0 clock");

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led: RedLED = pins.d13.into_open_drain_output(&mut pins.port);
    let rx: Sercom4Pad1<_> = pins
        .rx
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let tx: Sercom4Pad0<_> = pins
        .tx
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);

    let mut uart = UART4::new(
        &uart_clk,
        115200.hz(),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        (rx, tx),
    );

    // Write a string of bytes out the serial port
    for i in 0..MESSAGE.len()  {
        // NOTE `block!` blocks until `uart.write()` completes and returns
        // `Result<(), Error>`
        block!(uart.write(MESSAGE[i])).unwrap();
    }

    loop {
        red_led.set_high().unwrap();
        delay.delay_ms(1000u16);
        red_led.set_low().unwrap();
        delay.delay_ms(1000u16);
    }
}
