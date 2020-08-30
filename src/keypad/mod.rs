use crate::{
    config::*,
    view_id_gen::ViewIdGen,
    viewable::{ViewGroupId, Viewable},
};
use embedded_graphics::{
    pixelcolor::Rgb888, primitives::Rectangle, style::PrimitiveStyleBuilder, DrawTarget,
};
use embedded_layout::{
    layout::{
        linear::{spacing::Tight, LinearLayout},
        Guard, Link, ViewChainElement, ViewGroup,
    },
    prelude::*,
};

mod button;
mod key;

pub use button::{Button, ButtonView};
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
    pub fn new(vid_gen: &mut ViewIdGen) -> Self {
        Keypad {
            buttons: [
                Button::new(Key::One, vid_gen),
                Button::new(Key::Two, vid_gen),
                Button::new(Key::Three, vid_gen),
                Button::new(Key::Four, vid_gen),
                Button::new(Key::Five, vid_gen),
                Button::new(Key::Six, vid_gen),
                Button::new(Key::Seven, vid_gen),
                Button::new(Key::Eight, vid_gen),
                Button::new(Key::Nine, vid_gen),
                Button::new(Key::Asterisk, vid_gen),
                Button::new(Key::Zero, vid_gen),
                Button::new(Key::Pound, vid_gen),
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
}

impl<'a> Viewable<'a> for Keypad {
    type ViewType = KeypadView<'a>;

    fn view_group_id(&self) -> Option<ViewGroupId> {
        Some(DIALIER_VGID)
    }

    fn view(&'a self) -> Self::ViewType {
        KeypadView {
            inner: self,
            bounds: Rectangle::with_size(Point::zero(), KEYPAD_SIZE),
        }
    }
}

// RowChain, then combine them
//type Chain<'a> = chain! { ButtonView<'a>, ButtonView<'a>, ButtonView<'a> };

type Chain<'a> =
    Link<ViewGroup<Link<ButtonView<'a>, Link<ButtonView<'a>, Link<ButtonView<'a>>>>>, Guard>;

// some trait for this, StaticLayout
// type Chain = ...
impl<'a> KeypadView<'a> {
    // TODO - wip, experimenting with some sort of static layout / ViewGroup
    // move into KeypadView
    //
    //pub fn view_group<C: ViewChainElement, V: View>(&self) -> ViewGroup<Link<V, C>> {
    //
    pub fn view_group(&self) -> ViewGroup<Chain> {
        let primitive_style = PrimitiveStyleBuilder::new()
            .stroke_color(STROKE)
            .stroke_width(STROKE_WIDTH)
            .fill_color(FILL)
            .build();
        let primitive = self.bounds.into_styled(primitive_style);

        let row_0 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[0].view())
            .add_view(self.inner.buttons[1].view())
            .add_view(self.inner.buttons[2].view())
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Top);
        let row_1 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[3].view())
            .add_view(self.inner.buttons[4].view())
            .add_view(self.inner.buttons[5].view())
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Center);
        let row_2 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[6].view())
            .add_view(self.inner.buttons[7].view())
            .add_view(self.inner.buttons[8].view())
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Bottom);
        let row_3 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[9].view())
            .add_view(self.inner.buttons[10].view())
            .add_view(self.inner.buttons[11].view())
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Bottom);

        let layout = LinearLayout::vertical()
            .with_spacing(Tight)
            .add_view(row_0)
            //.add_view(row_1)
            //.add_view(row_2)
            //.add_view(row_3)
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Center);

        layout
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
            .add_view(self.inner.buttons[0].view())
            .add_view(self.inner.buttons[1].view())
            .add_view(self.inner.buttons[2].view())
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Top);
        let row_1 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[3].view())
            .add_view(self.inner.buttons[4].view())
            .add_view(self.inner.buttons[5].view())
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Center);
        let row_2 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[6].view())
            .add_view(self.inner.buttons[7].view())
            .add_view(self.inner.buttons[8].view())
            .arrange()
            .align_to(&primitive, horizontal::Center, vertical::Bottom);
        let row_3 = LinearLayout::horizontal()
            .with_spacing(Tight)
            .add_view(self.inner.buttons[9].view())
            .add_view(self.inner.buttons[10].view())
            .add_view(self.inner.buttons[11].view())
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
