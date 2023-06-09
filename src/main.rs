use embedded_graphics::mono_font::ascii::*;
use embedded_graphics::mono_font::iso_8859_14::FONT_8X13_ITALIC;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{DrawTarget, Point};
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::gpio::*;
use esp_idf_hal::i2c;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::FromValueType;
use ssd1306::prelude::DisplayConfig;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    //Buttons
    let mut button0 = PinDriver::input(peripherals.pins.gpio2)?;
    let mut button1 = PinDriver::input(peripherals.pins.gpio3)?;
    let mut button2 = PinDriver::input(peripherals.pins.gpio4)?;
    let mut button3 = PinDriver::input(peripherals.pins.gpio5)?;

    button0.set_pull(Pull::Down)?;
    button1.set_pull(Pull::Down)?;
    button2.set_pull(Pull::Down)?;
    button3.set_pull(Pull::Down)?;

    //i2c
    let mut sda = peripherals.pins.gpio6;
    let mut scl = peripherals.pins.gpio7;
    let mut _cfg = i2c::config::Config::new().baudrate(400.kHz().into());
    let i2c = i2c::I2cDriver::new(peripherals.i2c0, sda, scl, &_cfg).unwrap();

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X13_BOLD)
        .text_color(BinaryColor::On)
        .build();

    let mut text_line_0;
    let mut text_line_1;
    let mut text_line_2;
    let mut text_line_3;
    loop {
        display.clear_buffer();
        if button0.is_high() {
            text_line_0 = "0000"
        } else {
            text_line_0 = ""
        }

        if button1.is_high() {
            text_line_1 = "1111"
        } else {
            text_line_1 = ""
        }

        if button2.is_high() {
            text_line_2 = "2222"
        } else {
            text_line_2 = ""
        }

        if button3.is_high() {
            text_line_3 = "3333"
        } else {
            text_line_3 = ""
        }
        Text::with_baseline(&text_line_0, Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline(&text_line_1, Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline(&text_line_2, Point::new(0, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline(&text_line_3, Point::new(0, 48), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
    }
}
