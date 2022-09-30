//! Size provides additional functionality around sizing widgets
use crate::prelude::*;

/// Width provides dynamic screen calculations for width of widgets
#[derive(Debug, Copy, Clone)]
pub enum Width {
    /// Full width of the containing widget with optional (left, right) margin
    Full(f32, f32),

    /// Half width of the containing widget with optional (left, right) margin
    Half(f32, f32),

    /// Three quarter width of the containing widget with optional (left, right) margin
    ThreeQuarter(f32, f32),
}

impl Width {
    /// Calculate the width based on the width directive inside the given container
    /// * `container` is the containing widget's size to relatively position against
    pub fn relative(&self, container: Vec2) -> f32 {
        match self {
            Width::Full(l, r) => container.x - l - r,
            Width::Half(l, r) => container.x / 2.0 - l - r,
            Width::ThreeQuarter(l, r) => container.x * 2.0 / 3.0 - l - r,
        }
    }
}

/// Size provides dynamic screen calculations for sizing of widgets
#[derive(Debug, Copy, Clone)]
pub enum Size {
    /// Full width of screen with the given height
    /// * -1.0 for height value indicates full height
    FullWidth(f32),

    /// Half width of screen taking into account (margin, height)
    /// * -1.0 for height value indicates full height
    HalfWidth(f32, f32),

    /// Three quarter width of screen taking into account (margin, height)
    /// * -1.0 for height value indicates full height
    ThreeQuarter(f32, f32),

    /// Absolute width and height of the widget    
    Custom(f32, f32),
}

impl Size {
    /// Return the current screen size
    pub fn screen() -> Vec2 {
        vec2(screen_width(), screen_height())
    }

    /// Calculate the size vector based on the given component size
    pub fn vec2(&self) -> Vec2 {
        match self {
            Size::FullWidth(height) => {
                vec2(screen_width(), if *height == -1.0 { screen_height() } else { *height })
            },
            Size::HalfWidth(margin, height) => {
                vec2(screen_width() / 2.0 - margin, if *height == -1.0 { screen_height() } else { *height })
            },
            Size::ThreeQuarter(margin, height) => {
                vec2(screen_width() * 2.0 / 3.0 - margin, if *height == -1.0 { screen_height() } else { *height })
            },
            Size::Custom(width, height) => vec2(*width, *height),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::HalfWidth(5., 250.)
    }
}

impl From<Vec2> for Size {
    fn from(size: Vec2) -> Self {
        Self::Custom(size.x, size.y)
    }
}
