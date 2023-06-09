//! Blinks an LED
//!
//! This assumes that a LED is connected to GPIO4.
//! Depending on your target and the board you are using you should change the pin.
//! If your board doesn't have on-board LEDs don't forget to add an appropriate resistor.
//!

use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    //LEDs
    let mut led0 = PinDriver::output(peripherals.pins.gpio20)?;
    let mut led1 = PinDriver::output(peripherals.pins.gpio8)?;
    let mut led2 = PinDriver::output(peripherals.pins.gpio7)?;
    let mut led3 = PinDriver::output(peripherals.pins.gpio21)?;
    //Buttons
    let mut button0 = PinDriver::input(peripherals.pins.gpio2)?;
    let mut button1 = PinDriver::input(peripherals.pins.gpio3)?;
    let mut button2 = PinDriver::input(peripherals.pins.gpio4)?;
    let mut button3 = PinDriver::input(peripherals.pins.gpio5)?;

    button0.set_pull(Pull::Down)?;
    button1.set_pull(Pull::Down)?;
    button2.set_pull(Pull::Down)?;
    button3.set_pull(Pull::Down)?;

    loop {
        Delay::delay_ms(10);
        if button0.is_low() {
            print!("button is pressed");
            led0.set_high()?;
            led0.set_low()?;
        } else {
            led0.set_low()?;
            led0.set_high()?;
        }

        if button1.is_low() {
            print!("button is pressed");
            led1.set_high()?;
            led1.set_low()?;
        } else {
            led1.set_low()?;
            led1.set_high()?;
        }

        if button2.is_low() {
            print!("button is pressed");
            led2.set_high()?;
            led2.set_low()?;
        } else {
            led2.set_low()?;
            led2.set_high()?;
        }

        if button3.is_low() {
            print!("button is pressed");
            led3.set_high()?;
            led3.set_low()?;
        } else {
            led3.set_low()?;
            led3.set_high()?;
        }
    }
}
