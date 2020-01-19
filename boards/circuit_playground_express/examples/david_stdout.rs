//! David's UART stdout example for Circuit Playground Express
#![no_std]
#![no_main]

extern crate circuit_playground_express as hal;
extern crate cortex_m_rt;
extern crate panic_halt;

#[macro_use(block)]
extern crate nb;

use core::fmt::Write;
use cortex_m_rt::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::gclk::clkctrl::GEN_A;
use hal::pac::gclk::genctrl::SRC_A;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::{PadPin, Sercom4Pad0, Sercom4Pad1, UART4};

static MESSAGE: &[u8] = b"\r\nHello from SerialStdOut!\r\n";

// PA17 == samd21g18a pin 26 == Circuit Playground Express signal D13
type RedLED = hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>;

// Serial port set up using A6/A7 rx/tx pads
type SerialPort = UART4<
    Sercom4Pad1<hal::gpio::Pb9<hal::gpio::PfD>>,
    Sercom4Pad0<hal::gpio::Pb8<hal::gpio::PfD>>,
    (),
    (),
>;

struct ChipResources {
    pub delay: Delay,
    pub red_led: RedLED,
    pub stdout: SerialStdOut,
}

impl ChipResources {
    pub fn new() -> ChipResources {
        let mut peripherals = Peripherals::take().unwrap();
        let core = CorePeripherals::take().unwrap();

        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let delay = Delay::new(core.SYST, &mut clocks);

        let mut pins = hal::Pins::new(peripherals.PORT);
        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL48M, false);
        let gclk2 = clocks
            .get_gclk(GEN_A::GCLK2)
            .expect("Could not get clock 2");
        let uart_clk = clocks
            .sercom4_core(&gclk2)
            .expect("Could not configure sercom0 clock");
        let rx: Sercom4Pad1<_> = pins
            .rx
            .into_pull_down_input(&mut pins.port)
            .into_pad(&mut pins.port);
        let tx: Sercom4Pad0<_> = pins
            .tx
            .into_pull_down_input(&mut pins.port)
            .into_pad(&mut pins.port);
        let uart = UART4::new(
            &uart_clk,
            115200.hz(),
            peripherals.SERCOM4,
            &mut peripherals.PM,
            (rx, tx),
        );

        let red_led: RedLED = pins.d13.into_open_drain_output(&mut pins.port);
        let stdout = SerialStdOut::new(uart);

        ChipResources {
            delay,
            red_led,
            stdout,
        }
    }
}

struct SerialStdOut {
    uart: SerialPort,
}

impl SerialStdOut {
    pub fn new(uart: SerialPort) -> SerialStdOut {
        SerialStdOut { uart }
    }

    /// Write a slice of bytes out the serial port
    pub fn write_bytes(&mut self, msg: &[u8]) {
        for i in 0..msg.len() {
            block!(self.uart.write(msg[i])).unwrap();
        }
    }
}

#[entry]
fn main() -> ! {
    let mut chip = ChipResources::new();
    chip.stdout.write_bytes(MESSAGE);

    loop {
        chip.red_led.set_high().unwrap();
        chip.delay.delay_ms(1000u16);
        chip.red_led.set_low().unwrap();
        chip.delay.delay_ms(1000u16);
    }
}
