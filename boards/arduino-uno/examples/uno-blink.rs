#![no_std]
#![no_main]

use arduino_uno::{prelude::*, Peripherals, Pins};
use atmega328p_hal::{
    clock::MHz16,
    i2c::{Direction, I2cMaster},
};
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Connect to LED "L" and turn it off
    let mut led = pins.d13.into_output(&mut pins.ddr);
    led.set_low().void_unwrap();

    // Setup I²C-Controller
    let mut i2c = I2cMaster::<MHz16, _>::new(
        dp.TWI,
        pins.a4.into_pull_up_input(&mut pins.ddr),
        pins.a5.into_pull_up_input(&mut pins.ddr),
        400000, // what is `speed`?
    );

    // Ping peripheral (gy-86) and set LED to high in case of success
    if i2c.ping_slave(0x68, Direction::Read).unwrap() {
        led.set_high().void_unwrap();
    }

    loop {}
}
