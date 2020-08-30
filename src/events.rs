use embedded_graphics::prelude::*;
//use embedded_layout::prelude::*;

pub trait TouchEventHandler {
    //fn bounds(&self) -> Option<Rectangle>;

    // return something like ShouldForward/terminate
    fn handle_touch_event(&mut self, event: TouchEvent);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TouchEvent {
    Press(Point),
    Release(Point),
}
