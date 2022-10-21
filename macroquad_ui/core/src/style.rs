use macroquad::prelude::*;

/// Margin tracks offset space around and inside widgets and containers
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Margin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Margin {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { left, right, top, bottom }
    }
}
