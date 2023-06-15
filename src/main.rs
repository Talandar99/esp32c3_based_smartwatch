use embedded_graphics::mono_font::ascii::*;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{DrawTarget, Point, Size};
use embedded_graphics::primitives::{Circle, Primitive, PrimitiveStyleBuilder, Rectangle};
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
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    let text_style_small = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(BinaryColor::On)
        .build();

    let style_rectangle_selection = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    fn pixel2x2(
        x: u32,
        y: u32,
        display: &mut Ssd1306<
            ssd1306::prelude::I2CInterface<i2c::I2cDriver<'_>>,
            DisplaySize128x64,
            ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
        >,
    ) {
        display.set_pixel(x, y, true);
        display.set_pixel(x + 1, y, true);
        display.set_pixel(x, y + 1, true);
        display.set_pixel(x + 1, y + 1, true);
    }

    fn pixel4x4(
        x: u32,
        y: u32,
        display: &mut Ssd1306<
            ssd1306::prelude::I2CInterface<i2c::I2cDriver<'_>>,
            DisplaySize128x64,
            ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
        >,
    ) {
        display.set_pixel(x, y, true);
        display.set_pixel(x + 1, y, true);
        display.set_pixel(x + 2, y, true);
        display.set_pixel(x + 3, y, true);
        display.set_pixel(x, y + 1, true);
        display.set_pixel(x + 1, y + 1, true);
        display.set_pixel(x + 2, y + 1, true);
        display.set_pixel(x + 3, y + 1, true);
        display.set_pixel(x, y + 2, true);
        display.set_pixel(x + 1, y + 2, true);
        display.set_pixel(x + 2, y + 2, true);
        display.set_pixel(x + 3, y + 2, true);
        display.set_pixel(x, y + 3, true);
        display.set_pixel(x + 1, y + 3, true);
        display.set_pixel(x + 2, y + 3, true);
        display.set_pixel(x + 3, y + 3, true);
    }
    fn seven_seg_display(
        number: u32,
        display: &mut Ssd1306<
            ssd1306::prelude::I2CInterface<i2c::I2cDriver<'_>>,
            DisplaySize128x64,
            ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
        >,
        x: u32,
        y: u32,
    ) {
        if vec![2, 3, 4, 5, 6, 7, 8, 9, 0].contains(&number) {
            //A
            pixel4x4(0 + x, 4 + y, display);
            pixel4x4(4 + x, 4 + y, display);
            pixel4x4(8 + x, 4 + y, display);
            pixel4x4(12 + x, 4 + y, display);
        }

        if vec![1, 2, 3, 4, 7, 8, 9, 0].contains(&number) {
            //B
            pixel4x4(12 + x, 4 + y, display);
            pixel4x4(12 + x, 8 + y, display);
            pixel4x4(12 + x, 12 + y, display);
            pixel4x4(12 + x, 16 + y, display);
        }

        if vec![1, 3, 4, 5, 6, 7, 8, 9, 0].contains(&number) {
            //C
            pixel4x4(12 + x, 16 + y, display);
            pixel4x4(12 + x, 20 + y, display);
            pixel4x4(12 + x, 24 + y, display);
            pixel4x4(12 + x, 28 + y, display);
        }

        if vec![2, 3, 5, 6, 8, 9, 0].contains(&number) {
            //D
            pixel4x4(0 + x, 28 + y, display);
            pixel4x4(4 + x, 28 + y, display);
            pixel4x4(8 + x, 28 + y, display);
            pixel4x4(12 + x, 28 + y, display);
        }

        if vec![2, 6, 8, 0].contains(&number) {
            //E
            pixel4x4(0 + x, 16 + y, display);
            pixel4x4(0 + x, 20 + y, display);
            pixel4x4(0 + x, 24 + y, display);
            pixel4x4(0 + x, 28 + y, display);
        }

        if vec![4, 5, 6, 8, 9, 0].contains(&number) {
            //F
            pixel4x4(0 + x, 4 + y, display);
            pixel4x4(0 + x, 8 + y, display);
            pixel4x4(0 + x, 12 + y, display);
            pixel4x4(0 + x, 16 + y, display);
        }

        if vec![2, 3, 4, 5, 6, 8, 9].contains(&number) {
            //G
            pixel4x4(0 + x, 16 + y, display);
            pixel4x4(4 + x, 16 + y, display);
            pixel4x4(8 + x, 16 + y, display);
            pixel4x4(12 + x, 16 + y, display);
        }
    }

    loop {
        display.clear_buffer();
        if button0.is_high() {
            Rectangle::new(Point::new(0, 48), Size::new(32, 16))
                .into_styled(style_rectangle_selection)
                .draw(&mut display)
                .unwrap();
        }
        if button1.is_high() {
            Rectangle::new(Point::new(32, 48), Size::new(32, 16))
                .into_styled(style_rectangle_selection)
                .draw(&mut display)
                .unwrap();
        }
        if button2.is_high() {
            Rectangle::new(Point::new(64, 48), Size::new(32, 16))
                .into_styled(style_rectangle_selection)
                .draw(&mut display)
                .unwrap();
        }
        if button3.is_high() {
            Rectangle::new(Point::new(96, 48), Size::new(32, 16))
                .into_styled(style_rectangle_selection)
                .draw(&mut display)
                .unwrap();
        }

        seven_seg_display(2, &mut display, 16, 0);
        seven_seg_display(1, &mut display, 36, 0);
        pixel4x4(61, 8, &mut display);
        pixel4x4(61, 24, &mut display);
        seven_seg_display(3, &mut display, 74, 0);
        seven_seg_display(7, &mut display, 94, 0);

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(BinaryColor::On)
            .build();
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
        display.flush().unwrap();
    }
}
