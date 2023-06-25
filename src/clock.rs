use esp_idf_hal::i2c;
use ssd1306::{size::DisplaySize128x64, Ssd1306};

pub fn draw_2x7segment_time_display(
    display: &mut Ssd1306<
        ssd1306::prelude::I2CInterface<i2c::I2cDriver<'_>>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
    left: u32,
    right: u32,
) {
    let left0 = left / 10;
    let left1 = left % 10;
    let right0 = right / 10;
    let right1 = right % 10;
    single_7segment_display(left0, display, 16, 0);
    single_7segment_display(left1, display, 36, 0);
    pixel4x4(61, 8, display);
    pixel4x4(61, 24, display);
    single_7segment_display(right0, display, 74, 0);
    single_7segment_display(right1, display, 94, 0);
}

pub fn draw_3x7segment_time_display(
    display: &mut Ssd1306<
        ssd1306::prelude::I2CInterface<i2c::I2cDriver<'_>>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
    left: u32,
    middle: u32,
    right: u32,
) {
    let left0 = left / 10;
    let left1 = left % 10;
    let middle0 = middle / 10;
    let middle1 = middle % 10;
    let right0 = right / 10;
    let right1 = right % 10;
    single_7segment_display(left0, display, 0, 0);
    single_7segment_display(left1, display, 18, 0);
    pixel4x4(38, 8, display);
    pixel4x4(38, 24, display);
    single_7segment_display(middle0, display, 46, 0);
    single_7segment_display(middle1, display, 64, 0);
    pixel4x4(86, 8, display);
    pixel4x4(86, 24, display);
    single_7segment_display(right0, display, 94, 0);
    single_7segment_display(right1, display, 112, 0);
}

pub fn pixel2x2(
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

pub fn pixel4x4(
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
pub fn single_7segment_display(
    number: u32,
    display: &mut Ssd1306<
        ssd1306::prelude::I2CInterface<i2c::I2cDriver<'_>>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
    x: u32,
    y: u32,
) {
    if vec![2, 3, 5, 6, 7, 8, 9, 0].contains(&number) {
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
