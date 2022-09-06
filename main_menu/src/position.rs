//! Position provides additional functionality around positioning widgets
use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Position
{
    Center,
    Absolute(Vec2),
}

impl Default for Position
{
    fn default() -> Self
    {
        Position::Center
    }
}

impl From<Vec2> for Position
{
    fn from(position: Vec2) -> Self
    {
        Self::Absolute(position)
    }
}
