//! Configs

use crate::viewable::ViewGroupId;
use embedded_graphics::prelude::*;

pub const DISP_WIDTH: u32 = 720;
pub const DISP_HEIGHT: u32 = 1440;
//pub const DISP_CENTER: Point = Point::new(DISP_WIDTH as i32 / 2, DISP_HEIGHT as i32 / 2);

pub const BTN_SIZE: Size = Size::new((DISP_WIDTH / 3) - 8, DISP_WIDTH / 4);

pub const KEYPAD_SIZE: Size = Size::new(DISP_WIDTH - 2, BTN_SIZE.height * 4);

pub const DIALIER_VGID: ViewGroupId = 0;
