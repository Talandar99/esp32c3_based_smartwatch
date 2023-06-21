pub mod clock;

use clock::*;
use embedded_graphics::mono_font::ascii::*;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{DrawTarget, Point, Size};
use embedded_graphics::primitives::{Circle, Primitive, PrimitiveStyleBuilder, Rectangle};
use std::time::Duration;
use std::time::Instant;
//use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
//use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::gpio::*;
use esp_idf_hal::i2c;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::FromValueType;
use ssd1306::prelude::DisplayConfig;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};

struct Time {
    hours: u32,
    minutes: u32,
}
impl Time {
    fn set(&mut self, new_hours: u32, new_minutes: u32) {
        self.hours = new_hours;
        self.minutes = new_minutes;
    }
}
impl Time {
    fn increment_by_1_minute(&mut self) {
        if self.minutes >= 59 {
            self.minutes = 0;
            self.hours = self.hours + 1;
        } else {
            self.minutes = self.minutes + 1;
        }
        if self.hours > 23 {
            self.hours = 0;
        }
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    //Buttons
    let mut button0 = PinDriver::input(peripherals.pins.gpio2)?;
    let mut button1 = PinDriver::input(peripherals.pins.gpio3)?;
    let mut button2 = PinDriver::input(peripherals.pins.gpio21)?;
    let mut button3 = PinDriver::input(peripherals.pins.gpio20)?;
    button0.set_pull(Pull::Down)?;
    button1.set_pull(Pull::Down)?;
    button2.set_pull(Pull::Down)?;
    button3.set_pull(Pull::Down)?;
    let button_left = &button0;
    let button_down = &button1;
    let button_up = &button2;
    let button_right = &button3;
    //i2c
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio7;
    let mut _cfg = i2c::config::Config::new().baudrate(400.kHz().into());
    let i2c = i2c::I2cDriver::new(peripherals.i2c0, sda, scl, &_cfg).unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    //styles
    let text_style_small = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(BinaryColor::On)
        .build();
    let style_rectangle_selection = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();
    //time
    let mut clock_time = Time {
        hours: 0,
        minutes: 0,
    };

    clock_time.set(8, 17);
    let mut total_duration = Duration::new(0, 0);
    loop {
        let start_time = Instant::now();
        display.clear_buffer();
        draw_7seg_clock(&mut display, clock_time.hours, clock_time.minutes);
        if button_left.is_high() {
            Rectangle::new(Point::new(0, 48), Size::new(32, 16))
                .into_styled(style_rectangle_selection)
                .draw(&mut display)
                .unwrap();
        }
        if button_down.is_high() {
            Rectangle::new(Point::new(32, 48), Size::new(32, 16))
                .into_styled(style_rectangle_selection)
                .draw(&mut display)
                .unwrap();
        }
        if button_up.is_high() {
            Rectangle::new(Point::new(64, 48), Size::new(32, 16))
                .into_styled(style_rectangle_selection)
                .draw(&mut display)
                .unwrap();
        }
        if button_right.is_high() {
            Rectangle::new(Point::new(96, 48), Size::new(32, 16))
                .into_styled(style_rectangle_selection)
                .draw(&mut display)
                .unwrap();
        }

        display.flush().unwrap();
        let elapsed = start_time.elapsed();
        total_duration += elapsed;
        if total_duration >= Duration::from_secs(60) {
            total_duration -= Duration::from_secs(60);
            clock_time.increment_by_1_minute();
        }
    }
}
// this was in loop
//let style = PrimitiveStyleBuilder::new()
//    .stroke_width(1)
//    .stroke_color(BinaryColor::On)
//    .build();
//Rectangle::new(Point::new(16, 0), Size::new(94, 32))
//    .into_styled(style)
//    .draw(&mut display)
//    .unwrap();
//---------------------------------------------------
//Rectangle::new(Point::new(0, 0), Size::new(128, 36))
//    .into_styled(style)
//    .draw(&mut display)
//    .unwrap();
//---------------------------------------------------
//        Circle::new(Point::new(10, 10), 50)
//            .into_styled(
//                PrimitiveStyleBuilder::new()
//                    .stroke_width(2)
//                    .stroke_color(BinaryColor::On)
//                    .build(),
//            )
//            .draw(&mut display)
//            .unwrap();

//        Text::with_baseline("MENU", Point::new(4, 52), text_style_small, Baseline::Top)
//            .draw(&mut display)
//            .unwrap();
//        Text::with_baseline("", Point::new(36, 52), text_style_small, Baseline::Top)
//            .draw(&mut display)
//            .unwrap();
//        Text::with_baseline("", Point::new(68, 52), text_style_small, Baseline::Top)
//            .draw(&mut display)
//            .unwrap();
//        Text::with_baseline("LOCK", Point::new(100, 52), text_style_small, Baseline::Top)
//            .draw(&mut display)
//            .unwrap();
