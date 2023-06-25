pub mod clock;

use clock::*;
use embedded_graphics::mono_font::ascii::*;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{DrawTarget, Point, Size};
use embedded_graphics::primitives::{Circle, Primitive, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::Baseline;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use esp_idf_hal::gpio::*;
use esp_idf_hal::i2c;
use std::time::Duration;
use std::time::Instant;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::FromValueType;
use ssd1306::prelude::DisplayConfig;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};

struct Time {
    hours: u32,
    minutes: u32,
    seconds: u32,
}
impl Time {
    fn set(&mut self, new_hours: u32, new_minutes: u32, new_seconds: u32) {
        self.hours = new_hours;
        self.minutes = new_minutes;
        self.seconds = new_seconds;
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
    fn increment_by_1_second(&mut self) {
        if self.seconds >= 59 {
            self.seconds = 0;
            self.increment_by_1_minute();
        } else {
            self.seconds = self.seconds + 1;
        }
    }
}
fn draw_rectangle_buttons(
    display: &mut Ssd1306<
        ssd1306::prelude::I2CInterface<i2c::I2cDriver<'_>>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
    button_pressed: Vec<bool>,
) {
    let style_rectangle_selection = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();
    if button_pressed[0] {
        Rectangle::new(Point::new(0, 48), Size::new(32, 16))
            .into_styled(style_rectangle_selection)
            .draw(display)
            .unwrap();
    }

    if button_pressed[1] {
        Rectangle::new(Point::new(32, 48), Size::new(32, 16))
            .into_styled(style_rectangle_selection)
            .draw(display)
            .unwrap();
    }

    if button_pressed[2] {
        Rectangle::new(Point::new(64, 48), Size::new(32, 16))
            .into_styled(style_rectangle_selection)
            .draw(display)
            .unwrap();
    }

    if button_pressed[3] {
        Rectangle::new(Point::new(96, 48), Size::new(32, 16))
            .into_styled(style_rectangle_selection)
            .draw(display)
            .unwrap();
    }
}

fn navigation_button_bar(
    display: &mut Ssd1306<
        ssd1306::prelude::I2CInterface<i2c::I2cDriver<'_>>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
    button_pressed: Vec<bool>,
) {
    let text_style_small_on = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(BinaryColor::On)
        .build();
    let text_style_small_off = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(BinaryColor::Off)
        .build();

    let style_rectangle_selection = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .fill_color(BinaryColor::On)
        .build();

    for i in 0..128 {
        display.set_pixel(i, 50, true);
    }
    if button_pressed[0] {
        Rectangle::new(Point::new(0, 52), Size::new(32, 16))
            .into_styled(style_rectangle_selection)
            .draw(display)
            .unwrap();
        Text::with_baseline(
            "UNDO",
            Point::new(6, 54),
            text_style_small_off,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    } else {
        Text::with_baseline(
            "UNDO",
            Point::new(6, 54),
            text_style_small_on,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    }

    if button_pressed[1] {
        Rectangle::new(Point::new(32, 52), Size::new(32, 16))
            .into_styled(style_rectangle_selection)
            .draw(display)
            .unwrap();

        Text::with_baseline(
            "DOWN",
            Point::new(40, 54),
            text_style_small_off,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    } else {
        Text::with_baseline(
            "DOWN",
            Point::new(40, 54),
            text_style_small_on,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    }

    if button_pressed[2] {
        Rectangle::new(Point::new(64, 52), Size::new(32, 16))
            .into_styled(style_rectangle_selection)
            .draw(display)
            .unwrap();
        Text::with_baseline(
            " UP ",
            Point::new(70, 54),
            text_style_small_off,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    } else {
        Text::with_baseline(
            " UP ",
            Point::new(70, 54),
            text_style_small_on,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    }

    if button_pressed[3] {
        Rectangle::new(Point::new(96, 52), Size::new(32, 16))
            .into_styled(style_rectangle_selection)
            .draw(display)
            .unwrap();
        Text::with_baseline(
            " OK ",
            Point::new(100, 54),
            text_style_small_off,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    } else {
        Text::with_baseline(
            " OK ",
            Point::new(100, 54),
            text_style_small_on,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
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
    //time

    let mut clock_time = Time {
        hours: 0,
        minutes: 0,
        seconds: 0,
    };

    clock_time.set(21, 37, 00);
    let mut total_duration = Duration::new(0, 0);
    let mut button_pressed: Vec<bool>;
    loop {
        button_pressed = vec![false, false, false, false];
        let start_time = Instant::now();
        display.clear_buffer();
        if button_left.is_high() {
            button_pressed[0] = true;
        }
        if button_down.is_high() {
            button_pressed[1] = true;
        }
        if button_up.is_high() {
            button_pressed[2] = true;
        }
        if button_right.is_high() {
            button_pressed[3] = true;
        }

        navigation_button_bar(&mut display, button_pressed);
        draw_3x7segment_time_display(
            &mut display,
            clock_time.hours,
            clock_time.minutes,
            clock_time.seconds,
        );
        display.flush().unwrap();
        //updating time
        let elapsed = start_time.elapsed();
        total_duration += elapsed;
        if total_duration >= Duration::from_secs(1) {
            total_duration -= Duration::from_secs(1);
            clock_time.increment_by_1_second();
        }

        //        if total_duration >= Duration::from_secs(60) {
        //            total_duration -= Duration::from_secs(60);
        //            clock_time.increment_by_1_minute();
        //        }
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
