use crate::prelude::*;

pub trait Widget {
    fn layout_ref(&self) -> Layout;
}
