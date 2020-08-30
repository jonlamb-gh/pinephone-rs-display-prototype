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

    fn view(&'a self) -> Self::ViewType;
}
