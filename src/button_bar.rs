use embedded_graphics::{
    mono_font::{ascii::*, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::{Primitive, PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text},
    Drawable,
};
use esp_idf_hal::i2c;
use ssd1306::{size::DisplaySize128x64, Ssd1306};

pub fn draw_rectangle_buttons(
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

pub fn navigation_button_bar(
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
