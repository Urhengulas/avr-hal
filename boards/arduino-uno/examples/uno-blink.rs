#![no_std]
#![no_main]

use arduino_uno::{
    hal::{
        clock::MHz16,
        i2c::{Direction, I2cMaster},
    },
    prelude::*,
    Peripherals, Pins,
};
use panic_halt as _;

const IMU_ADDR: u8 = 0x68;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Connect to LED "L" and turn it off
    let mut led = pins.d13.into_output(&mut pins.ddr);
    led.set_low().void_unwrap();

    // Set up serial connection
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    // Setup I²C-Controller
    let mut i2c = I2cMaster::<MHz16, _>::new(
        dp.TWI,
        pins.a4.into_pull_up_input(&mut pins.ddr),
        pins.a5.into_pull_up_input(&mut pins.ddr),
        400000, // what is `speed`?
    );

    // Ping peripheral (gy-86) and set LED to high in case of success
    if i2c.ping_slave(IMU_ADDR, Direction::Read).unwrap() {
        led.set_high().void_unwrap();
    }

    // wake MPU6050 up
    i2c.write(IMU_ADDR, &[0x6B, 0]).unwrap();

    // read data
    let mut buf = [0; 14];
    i2c.write_read(IMU_ADDR, &[0x3B], &mut buf).unwrap();

    // convert data
    // in the end we are always appending the second byte to the end of the first
    // mathematically this operation is: a * 2^8 + b
    let mut data = [0; 7];
    for (idx, chunk) in buf.chunks_exact(2).enumerate() {
        let a = chunk[0] as u16;
        let b = chunk[1] as u16;
        data[idx] = a << 8 | b;
    }

    // report result over serial
    ufmt::uwrite!(&mut serial, "Read: ").void_unwrap();
    for i in data.iter() {
        ufmt::uwrite!(&mut serial, " {},", i).void_unwrap();
    }
    ufmt::uwrite!(&mut serial, "\r\n").void_unwrap();

    loop {}
}
