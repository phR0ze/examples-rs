//! Size provides additional functionality around sizing widgets
use macroquad::prelude::*;

/// Width provides dynamic screen calculations for width of widgets
#[derive(Debug, Copy, Clone)]
pub enum Width {
    /// Full width of screen with optional margin
    Full(Option<RectOffset>),

    /// Half width of screen with optional margin
    Half(Option<RectOffset>),

    /// Three quarter width of screen with optional margin
    ThreeQuarter(Option<RectOffset>),
}

impl Width {
    /// Calculate the width
    pub fn f32(&self) -> f32 {
        match self {
            Width::Full(None) => screen_width(),
            Width::Full(Some(margin)) => screen_width() - margin.left - margin.right,
            Width::Half(None) => screen_width() / 2.0,
            Width::Half(Some(margin)) => (screen_width() - margin.left - margin.right) / 2.0,
            Width::ThreeQuarter(None) => screen_width() * 2.0 / 3.0,
            Width::ThreeQuarter(Some(margin)) => (screen_width() - margin.left - margin.right) * 2.0 / 3.0,
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
    Absolute(f32, f32),
}

impl Size {
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
            Size::Absolute(width, height) => vec2(*width, *height),
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
        Self::Absolute(size.x, size.y)
    }
}
