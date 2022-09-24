//! Size provides additional functionality around sizing widgets
use macroquad::prelude::*;

/// Size provides dynamic screen calculations for sizing of widgets
/// * -1 for height or width value indicates full height
#[derive(Debug, Copy, Clone)]
pub enum Size {
    HalfWidth(f32, f32), // calculates half of the screen width taking into account (margin, height)
    ThreeQuarter(f32, f32), // calculates three quarter of the screen width taking into account (margin, height)
    Absolute(Vec2),
}

impl Size {
    /// Calculate the size vector based on the given component size
    pub fn vec2(&self) -> Vec2 {
        match self {
            Size::HalfWidth(margin, height) => vec2(screen_width() / 2.0 - margin, *height),
            Size::ThreeQuarter(margin, height) => {
                vec2(screen_width() * 2.0 / 3.0 - margin, if *height == -1.0 { screen_height() } else { *height })
            },
            Size::Absolute(size) => *size,
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
        Self::Absolute(size)
    }
}
