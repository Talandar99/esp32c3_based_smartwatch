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

    button0.set_pull(Pull::Up)?;
    //button1.set_pull(Pull::Up)?;

    loop {
        //  Delay::delay_ms(10);
        //  if button0.is_high() {
        //      led0.set_high()?;
        //      led1.set_low()?;
        //      //FreeRtos::delay_ms(200);
        //  } else {
        //      led0.set_low()?;
        //      led0.set_high()?;
        //      //FreeRtos::delay_ms(200);
        //  }

        led0.set_high()?;
        FreeRtos::delay_ms(200);
        led0.set_low()?;
        FreeRtos::delay_ms(200);
        led1.set_high()?;
        FreeRtos::delay_ms(200);
        led1.set_low()?;
        FreeRtos::delay_ms(200);
        led2.set_high()?;
        FreeRtos::delay_ms(200);
        led2.set_low()?;
        FreeRtos::delay_ms(200);
        led3.set_high()?;
        FreeRtos::delay_ms(200);
        led3.set_low()?;
        FreeRtos::delay_ms(200);
    }
}
