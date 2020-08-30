use crate::keypad::Key;
use embedded_graphics::{
    fonts::{Font24x32, Text},
    pixelcolor::Rgb888,
    primitives::Rectangle,
    style::{PrimitiveStyleBuilder, TextStyleBuilder},
    DrawTarget,
};
use embedded_layout::prelude::*;

const STROKE_WIDTH: u32 = 4;
const STROKE: Rgb888 = Rgb888::new(64, 64, 64);
const FILL: Rgb888 = Rgb888::WHITE;
const FILL_DEPRESSED: Rgb888 = Rgb888::new(128, 128, 128);
const TEXT: Rgb888 = Rgb888::BLACK;
const TEXT_FONT: Font24x32 = Font24x32;

#[derive(Debug)]
pub struct Button {
    key: Key,
    depressed: bool,
}

pub struct ButtonView<'a> {
    inner: &'a Button,
    bounds: Rectangle,
}

impl Button {
    pub fn new(key: Key) -> Self {
        Button {
            key,
            depressed: false,
        }
    }

    pub fn set_depressed(&mut self, depressed: bool) {
        self.depressed = depressed;
    }

    pub fn view(&self, position: Point, size: Size) -> ButtonView {
        ButtonView {
            inner: self,
            bounds: Rectangle::with_size(position, size),
        }
    }
}

impl<'a> View for ButtonView<'a> {
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

impl<'a> Drawable<Rgb888> for &ButtonView<'a> {
    fn draw<D: DrawTarget<Rgb888>>(self, display: &mut D) -> Result<(), D::Error> {
        let fill_color = if self.inner.depressed {
            FILL_DEPRESSED
        } else {
            FILL
        };
        let primitive_style = PrimitiveStyleBuilder::new()
            .stroke_color(STROKE)
            .stroke_width(STROKE_WIDTH)
            .fill_color(fill_color)
            .build();
        let primitive = self.bounds.into_styled(primitive_style);

        let text_style = TextStyleBuilder::new(TEXT_FONT)
            .text_color(TEXT)
            .background_color(fill_color)
            .build();

        let text = Text::new(self.inner.key.as_str(), Point::zero())
            .into_styled(text_style)
            .align_to(&primitive, horizontal::Center, vertical::Center);

        primitive.draw(display)?;
        text.draw(display)?;

        Ok(())
    }
}
