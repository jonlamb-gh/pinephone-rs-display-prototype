use embedded_layout::prelude::*;

/// All views in a ViewGroup should shared a common ViewGroupId.
///
/// The active/in-focus ViewGroupId handles events.
pub type ViewGroupId = usize;

pub trait Viewable<'a> {
    type ViewType: View;

    fn view_group_id(&self) -> Option<ViewGroupId> {
        None
    }

    // TODO - point is always Zero here, size can be config/const
    fn view(&'a self, position: Point, size: Size) -> Self::ViewType;
}
