pub mod button_bar;
pub mod clock;

use button_bar::*;
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
enum View {
    clock_hours_minutes,
    clock_hours_minutes_seconds,
    menu,
    settings,
}
#[derive(Clone)]
struct MenuState {
    selected_option: i8,
    all_options: Vec<String>,
}

fn update_menu_state(button_pressed: Vec<bool>, old_menu_state: MenuState) -> MenuState {
    if button_pressed[1] | button_pressed[2] {
        let mut new_menu_state = old_menu_state.clone();
        if button_pressed[1] {
            if new_menu_state.selected_option < new_menu_state.all_options.len() as i8 {
                new_menu_state.selected_option = new_menu_state.selected_option + 1;
            }
        }
        if button_pressed[2] {
            if new_menu_state.selected_option > 0 {
                new_menu_state.selected_option = new_menu_state.selected_option - 1;
            }
        }
        return new_menu_state;
    } else {
        return old_menu_state;
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
        .font(&FONT_6X10)
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
    //loop Variables
    let mut button_pressed: Vec<bool>;
    let mut selected_view: View = View::clock_hours_minutes;
    let mut menu_state = MenuState {
        selected_option: 0,
        all_options: vec![
            "First".to_string(),
            "Secon".to_string(),
            "Third".to_string(),
            "Another".to_string(),
            "AAAno".to_string(),
            "zczxc".to_string(),
            "sdasd".to_string(),
        ],
    };

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

        match selected_view {
            View::clock_hours_minutes_seconds => {
                draw_3x7segment_time_display(
                    &mut display,
                    clock_time.hours,
                    clock_time.minutes,
                    clock_time.seconds,
                );
                navigation_button_bar(&mut display, button_pressed.clone());
                if button_pressed[3].clone() {
                    selected_view = View::menu;
                }
            }
            View::clock_hours_minutes => {
                draw_2x7segment_time_display(&mut display, clock_time.hours, clock_time.minutes);
                navigation_button_bar(&mut display, button_pressed.clone());
                if button_pressed[3].clone() {
                    selected_view = View::menu;
                }
            }
            View::menu => {
                menu_state = update_menu_state(button_pressed.clone(), menu_state);
                //TODO drawing menu
                navigation_button_bar(&mut display, button_pressed.clone());
            }
            View::settings => {
                navigation_button_bar(&mut display, button_pressed);
            }
        }
        display.flush().unwrap();
        //updating time
        let elapsed = start_time.elapsed();
        total_duration += elapsed;
        if total_duration >= Duration::from_secs(1) {
            total_duration -= Duration::from_secs(1);
            clock_time.increment_by_1_second();
        }
        // need more testing
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
