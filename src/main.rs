// https://github.com/bugadani/embedded-layout
//
// https://docs.rs/embedded-layout/0.1.0/embedded_layout/
//
// https://github.com/bugadani/embedded-layout/blob/master/examples/custom_view.rs
//
// https://github.com/jamwaffles/embedded-graphics/blob/master/simulator/examples/input-handling.rs
//
// https://github.com/jamwaffles/embedded-graphics/blob/master/simulator/examples/analog-clock.rs
//
// https://docs.rs/embedded-graphics/0.6.2/embedded_graphics/index.html
//
// https://docs.rs/embedded-graphics-simulator/0.2.1/embedded_graphics_simulator/

// need an event system and absolute position to object mappings for the lookup

// dialer
//   text-bar for the digits
//   btns: phonebook | dial | delete
//   below is a keypad of buttons
//   something like: https://www.mobileappdaily.com/public/uploads/mad_89583ec748.png
//
// phonebook

use crate::config::*;
use crate::keypad::{Key, Keypad};
use crate::viewable::Viewable;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_layout::{
    layout::linear::{spacing::FixedMargin, LinearLayout},
    prelude::*,
};

mod config;
mod events;
mod keypad;
mod viewable;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(DISP_WIDTH, DISP_HEIGHT));
    let display_area = display.display_area();

    let output_settings = OutputSettingsBuilder::new()
        .scale(1)
        .theme(BinaryColorTheme::Default)
        .pixel_spacing(0)
        .build();
    let mut window = Window::new("pinephone-rs prototype", &output_settings);

    let mut keypad = Keypad::new();

    'running: loop {
        let layout = LinearLayout::vertical()
            .with_spacing(FixedMargin(4))
            .add_view(keypad.view())
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Bottom);

        layout.draw(&mut display)?;

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    println!("{:?}", point);
                    keypad.set_depressed(Key::Asterisk, true);
                }
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    println!("{:?}", point);
                    keypad.set_depressed(Key::Asterisk, false);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
