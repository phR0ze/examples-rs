//! Size provides additional functionality around sizing widgets
use crate::prelude::*;

/// Height provides dynamic screen calculations for height of widgets
#[derive(Debug, Copy, Clone)]
pub enum Height {
    /// Full height of the containing widget with (top, bottom) margins
    Full(Option<(f32, f32)>),

    /// Half height of the containing widget with (top, bottom) margins
    Half(Option<(f32, f32)>),

    /// Three quarter height of the containing widget with (top, bottom) margins
    ThreeQuarter(Option<(f32, f32)>),

    /// Percentage of the containing widget's height with valid range of 0.01 - 1.0 Values
    /// outside this range will be automatically converted to the closet bound.
    Percent(f32),
}

impl Height {
    /// Calculate the height based on the height directive inside the given container
    /// * `container` is the containing widget's size to relatively position against
    pub fn relative(&self, container: Vec2) -> f32 {
        match self {
            Height::Full(None) => container.y,
            Height::Full(Some((t, b))) => container.y - t - b,
            Height::Half(None) => container.y / 2.0,
            Height::Half(Some((t, b))) => container.y / 2.0 - t - b,
            Height::ThreeQuarter(None) => container.y * 2.0 / 3.0,
            Height::ThreeQuarter(Some((t, b))) => container.y * 2.0 / 3.0 - t - b,
            Height::Percent(y) => {
                container.y
                    * if *y < 0. {
                        0.01
                    } else if *y > 1.0 {
                        1.0
                    } else {
                        *y
                    }
            },
        }
    }
}

/// Width provides dynamic screen calculations for width of widgets
#[derive(Debug, Copy, Clone)]
pub enum Width {
    /// Full width of the containing widget with (left, right) margins
    Full(Option<(f32, f32)>),

    /// Half width of the containing widget with (left, right) margins
    Half(Option<(f32, f32)>),

    /// Three quarter width of the containing widget with (left, right) margins
    ThreeQuarter(Option<(f32, f32)>),

    /// Percentage of the containing widget's width with valid range of 0.01 - 1.0 Values
    /// outside this range will be automatically converted to the closet bound.
    Percent(f32),
}

impl Width {
    /// Calculate the width based on the width directive inside the given container
    /// * `container` is the containing widget's size to relatively position against
    pub fn relative(&self, container: Vec2) -> f32 {
        match self {
            Width::Full(None) => container.x,
            Width::Full(Some((l, r))) => container.x - l - r,
            Width::Half(None) => container.x / 2.0,
            Width::Half(Some((l, r))) => container.x / 2.0 - l - r,
            Width::ThreeQuarter(None) => container.x * 2.0 / 3.0,
            Width::ThreeQuarter(Some((l, r))) => container.x * 2.0 / 3.0 - l - r,
            Width::Percent(x) => {
                container.x
                    * if *x < 0. {
                        0.01
                    } else if *x > 1.0 {
                        1.0
                    } else {
                        *x
                    }
            },
        }
    }
}

/// Size provides dynamic screen calculations for sizing of widgets
#[derive(Debug, Copy, Clone)]
pub enum Size {
    /// Calculated based on width and height directives
    Calc(Width, Height),

    // Calculated height and static width
    CalcHeight(f32, Height),

    // Calculated width and static height
    CalcWidth(Width, f32),

    /// Calculated 3/4 the size of the container
    ThreeQuarter,

    /// Percentage of the containing widget's (width, height) with valid range of 0.01 - 1.0
    /// Values outside this range will be automatically converted to the closet bound.
    Percent(f32, f32),

    /// Static width and height
    Static(f32, f32),
}

impl Size {
    /// Return the current screen size
    pub fn screen() -> Vec2 {
        vec2(screen_width(), screen_height())
    }

    /// Calculate the size vector based on the given containing widget's size
    /// * `container` is the containing widget's size
    pub fn relative(&self, container: Vec2) -> Vec2 {
        match self {
            Size::Calc(w, h) => vec2(w.relative(container), h.relative(container)),
            Size::CalcHeight(w, h) => vec2(*w, h.relative(container)),
            Size::CalcWidth(w, h) => vec2(w.relative(container), *h),
            Size::ThreeQuarter => {
                vec2(Width::ThreeQuarter(None).relative(container), Height::ThreeQuarter(None).relative(container))
            },
            Size::Percent(w, h) => {
                vec2(Width::Percent(*w).relative(container), Height::Percent(*h).relative(container))
            },
            Size::Static(w, h) => vec2(*w, *h),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::Percent(0.50, 0.50)
    }
}
