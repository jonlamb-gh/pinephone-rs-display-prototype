use embedded_layout::ViewId;

pub struct ViewIdGen {
    next: ViewId,
}

impl ViewIdGen {
    pub fn new() -> Self {
        ViewIdGen { next: 0 }
    }

    pub fn next(&mut self) -> ViewId {
        let next = self.next;
        self.next += 1;
        next
    }
}
