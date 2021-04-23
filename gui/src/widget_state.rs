use std::rc::Rc;

use std::cell::{Cell, RefCell};
use crate::border::{Border};
use crate::widget_operation::DirtyFlags;
use crate::background::{Background};
use crate::text_style::TextStyle;

pub struct WidgetState {
    pub dirty_flags: Cell<DirtyFlags>,
    pub text_style: RefCell<Option<Rc<TextStyle>>>,
    pub background: RefCell<Option<Rc<Background>>>,
    pub border: RefCell<Option<Rc<Border>>>,
}

impl WidgetState {
    pub fn new() -> Self {
        Self {
            dirty_flags: Cell::new(DirtyFlags::ALL),
            text_style: RefCell::new(None),
            background: RefCell::new(None),
            border: RefCell::new(None),
        }
    }

    pub fn dirty_flag_clean(&self, flag: DirtyFlags) -> bool {
        let mut dirty = (&self.dirty_flags).get();
        if (dirty & flag).is_empty() {
            return true;
        }
        dirty.remove(flag);

        return false;
    }
}
