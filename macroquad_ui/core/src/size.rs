//! Size provides additional functionality around sizing widgets
use crate::prelude::*;

/// Height provides dynamic screen calculations for height of widgets
#[derive(Debug, Copy, Clone)]
pub enum Height {
    /// Dynamically determine height based on content size with
    Dynamic,

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
    /// * `content` is the size of the content and only used for Height::Dynamic
    pub fn relative(&self, container: Vec2, content: Option<Vec2>) -> f32 {
        match self {
            Height::Dynamic => content.unwrap_or(empty()).y,
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

    /// Scale the height value for mobile use
    pub fn scale(&self) -> Height {
        match self {
            Height::Dynamic => Height::Dynamic,
            Height::Full(None) => Height::Full(None),
            Height::Full(Some((t, b))) => Height::Full(Some((scale(*t), scale(*b)))),
            Height::Half(None) => Height::Half(None),
            Height::Half(Some((t, b))) => Height::Half(Some((scale(*t), scale(*b)))),
            Height::ThreeQuarter(None) => Height::ThreeQuarter(None),
            Height::ThreeQuarter(Some((t, b))) => Height::ThreeQuarter(Some((scale(*t), scale(*b)))),
            Height::Percent(y) => Height::Percent(*y),
        }
    }
}

/// Width provides dynamic screen calculations for width of widgets
#[derive(Debug, Copy, Clone)]
pub enum Width {
    /// Dynamically determine required width based on content size
    Dynamic,

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
    /// * `content` is the size of the content and only used for Height::Dynamic
    pub fn relative(&self, container: Vec2, content: Option<Vec2>) -> f32 {
        match self {
            Width::Dynamic => content.unwrap_or(empty()).x,
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

    /// Scale the width value for mobile use
    pub fn scale(&self) -> Width {
        match self {
            Width::Dynamic => Width::Dynamic,
            Width::Full(None) => Width::Full(None),
            Width::Full(Some((l, r))) => Width::Full(Some((scale(*l), scale(*r)))),
            Width::Half(None) => Width::Half(None),
            Width::Half(Some((l, r))) => Width::Half(Some((scale(*l), scale(*r)))),
            Width::ThreeQuarter(None) => Width::ThreeQuarter(None),
            Width::ThreeQuarter(Some((l, r))) => Width::ThreeQuarter(Some((scale(*l), scale(*r)))),
            Width::Percent(x) => Width::Percent(*x),
        }
    }
}

/// Size provides dynamic screen calculations for sizing of widgets
#[derive(Debug, Copy, Clone)]
pub enum Size {
    /// Dynamically determine required width and height based on content size
    Dynamic,

    /// Calculated based on width and height directives
    Calc(Width, Height),

    // Calculated height and static width
    CalcHeight(f32, Height),

    // Calculated width and dynamic height
    CalcWidth(Width, f32),

    /// Percentage of the containing widget's (width, height) with valid range of 0.01 - 1.0
    /// Values outside this range will be automatically converted to the closet bound.
    Percent(f32, f32),

    /// Static width and height
    Static(f32, f32),
}

impl Size {
    /// Set width to 1/2 of the horizontal space and height dynamic
    pub fn half_width() -> Size {
        Size::Calc(Width::Half(None), Height::Dynamic)
    }

    /// Set width to 3/4 of the horizontal space and height dynamic
    pub fn three_quarter_width() -> Size {
        Size::Calc(Width::ThreeQuarter(None), Height::Dynamic)
    }

    /// Calculate the size vector based on the given containing widget's size
    /// * `container` is the containing widget's size
    /// * `content` is the size of the content and only used for Height::Dynamic
    pub fn relative(&self, container: Vec2, content: Option<Vec2>) -> Vec2 {
        match self {
            Size::Dynamic => content.unwrap_or(empty()),
            Size::Calc(w, h) => vec2(w.relative(container, content), h.relative(container, content)),
            Size::CalcHeight(w, h) => vec2(*w, h.relative(container, content)),
            Size::CalcWidth(w, h) => vec2(w.relative(container, content), *h),
            Size::Percent(w, h) => vec2(
                Width::Percent(*w).relative(container, content),
                Height::Percent(*h).relative(container, content),
            ),
            Size::Static(w, h) => vec2(*w, *h),
        }
    }

    /// Scale the size values for mobile use
    pub fn scale(&self) -> Size {
        match self {
            Size::Dynamic => Size::Dynamic,
            Size::Calc(w, h) => Size::Calc(w.scale(), h.scale()),
            Size::CalcHeight(w, h) => Size::CalcHeight(scale(*w), h.scale()),
            Size::CalcWidth(w, h) => Size::CalcWidth(w.scale(), scale(*h)),
            Size::Percent(w, h) => Size::Percent(*w, *h),
            Size::Static(w, h) => Size::Static(scale(*w), scale(*h)),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::Dynamic
    }
}
