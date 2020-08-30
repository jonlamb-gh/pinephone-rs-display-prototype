use embedded_graphics::{
    pixelcolor::Rgb888, primitives::Rectangle, style::PrimitiveStyleBuilder, DrawTarget,
};
use embedded_layout::{
    layout::linear::{spacing::Tight, LinearLayout},
    prelude::*,
};

mod button;
mod key;

use crate::config::*;
pub use button::Button;
pub use key::Key;

const STROKE_WIDTH: u32 = 4;
const STROKE: Rgb888 = Rgb888::new(64, 64, 64);
const FILL: Rgb888 = Rgb888::BLACK;

#[derive(Debug)]
pub struct Keypad {
    buttons: [Button; 12],
}

#[derive(Debug)]
pub struct KeypadView<'a> {
    inner: &'a Keypad,
    bounds: Rectangle,
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            buttons: [
                Button::new(Key::One),
                Button::new(Key::Two),
                Button::new(Key::Three),
                Button::new(Key::Four),
                Button::new(Key::Five),
                Button::new(Key::Six),
                Button::new(Key::Seven),
                Button::new(Key::Eight),
                Button::new(Key::Nine),
                Button::new(Key::Asterisk),
                Button::new(Key::Zero),
                Button::new(Key::Pound),
            ],
        }
    }

    pub fn set_depressed(&mut self, key: Key, depressed: bool) {
        for b in self.buttons.iter_mut() {
            if b.key() == key {
                b.set_depressed(depressed);
            }
        }
    }

    pub fn view(&self, position: Point, size: Size) -> KeypadView {
        KeypadView {
            inner: self,
            bounds: Rectangle::with_size(position, size),
        }
    }
}

impl<'a> View for KeypadView<'a> {
    #[inline]
    fn translate(mut self, by: Point) -> Self {
        self.translate_mut(by);
        self
    }

    #[inline]
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.bounds.translate_mut(by);
        self
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

impl<'a> Drawable<Rgb888> for &KeypadView<'a> {
    fn draw<D: DrawTarget<Rgb888>>(self, display: &mut D) -> Result<(), D::Error> {
        let primitive_style = PrimitiveStyleBuilder::new()
            .stroke_color(STROKE)
            .stroke_width(STROKE_WIDTH)
            .fill_color(FILL)
            .build();
        let primitive = self.bounds.into_styled(primitive_style);

        let row_0 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[0].view(Point::zero(), BTN_SIZE))
            .add_view(self.inner.buttons[1].view(Point::zero(), BTN_SIZE))
            .add_view(self.inner.buttons[2].view(Point::zero(), BTN_SIZE))
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Top);
        let row_1 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[3].view(Point::zero(), BTN_SIZE))
            .add_view(self.inner.buttons[4].view(Point::zero(), BTN_SIZE))
            .add_view(self.inner.buttons[5].view(Point::zero(), BTN_SIZE))
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Center);
        let row_2 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[6].view(Point::zero(), BTN_SIZE))
            .add_view(self.inner.buttons[7].view(Point::zero(), BTN_SIZE))
            .add_view(self.inner.buttons[8].view(Point::zero(), BTN_SIZE))
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Bottom);
        let row_3 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[9].view(Point::zero(), BTN_SIZE))
            .add_view(self.inner.buttons[10].view(Point::zero(), BTN_SIZE))
            .add_view(self.inner.buttons[11].view(Point::zero(), BTN_SIZE))
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Bottom);

        let layout = LinearLayout::vertical()
            .with_spacing(Tight)
            .add_view(row_0)
            .add_view(row_1)
            .add_view(row_2)
            .add_view(row_3)
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Center);

        primitive.draw(display)?;
        layout.draw(display)?;

        Ok(())
    }
}
