//! Position provides additional functionality around positioning widgets
use macroquad::prelude::*;

use crate::utils::scale;

/// Position is a directive used to calculate the actual position. The enum values are
/// named in a WidthHeight camel case describing the width and height positioning directive.
#[derive(Debug, Copy, Clone)]
pub enum Position {
    Center,

    CenterTop,

    /// Position on the right horizontally and center vertically
    /// * (right) optional margin value can be set to offset
    RightCenter(f32),

    /// Position on the left horizontally and center vertically
    /// * (left) optional margin value can be set to offset
    LeftCenter(f32),

    /// Position on the left horizontally and top vertically
    /// * (left, top) optional margin values can be set to offset
    LeftTop(f32, f32),

    /// Custom position relative to the containing widget
    Custom(f32, f32),
}

impl Position {
    /// Position of origin
    pub fn origin() -> Vec2 {
        vec2(0.0, 0.0)
    }

    /// Scale the positional margins
    pub fn scale(&self) -> Position {
        match self {
            Position::Center => Position::Center,
            Position::CenterTop => Position::CenterTop,
            Position::RightCenter(right) => Position::RightCenter(scale(*right)),
            Position::LeftCenter(left) => Position::LeftCenter(scale(*left)),
            Position::LeftTop(left, top) => Position::LeftTop(scale(*left), scale(*top)),
            Position::Custom(x, y) => Position::Custom(*x, *y),
        }
    }

    /// Calculate the position vector based on the given widget size and the
    /// containing widget size and the positioning directive.
    /// * `target` is the size of the target component to position
    /// * `container` is the containing widget's size
    pub fn relative(&self, target: Vec2, container: Vec2) -> Vec2 {
        match self {
            Position::Center => vec2(container.x - target.x, container.y - target.y) / 2.0,
            Position::CenterTop => vec2((container.x - target.x) / 2.0, 0.0),
            Position::RightCenter(right) => vec2(
                container.x - target.x - if *right > 0. { *right } else { 0. },
                (container.y - target.y) / 2.0,
            ),
            Position::LeftCenter(left) => {
                vec2(if *left > 0. { *left } else { 0. }, (container.y - target.y) / 2.0)
            },
            Position::LeftTop(left, top) => {
                vec2(if *left > 0. { *left } else { 0. }, if *top > 0. { *top } else { 0. })
            },
            Position::Custom(x, y) => vec2(*x, *y),
        }
    }

    /// Calculate the position vector based on the given component size
    /// * `target` is the component's size to be taken into account
    pub fn vec2(&self, target: Vec2) -> Vec2 {
        let container = vec2(screen_width(), screen_height());
        match self {
            Position::Center => Position::Center.relative(target, container),
            Position::CenterTop => Position::CenterTop.relative(target, container),
            Position::RightCenter(right) => Position::RightCenter(*right).relative(target, container),
            Position::LeftCenter(left) => Position::LeftCenter(*left).relative(target, container),
            Position::LeftTop(left, top) => Position::LeftTop(*left, *top).relative(target, container),
            Position::Custom(x, y) => vec2(*x, *y),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::Center
    }
}
