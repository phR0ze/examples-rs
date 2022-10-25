use crate::prelude::*;

/// Layout describes a region of space and provides mechanisms for calculating where and how a
/// widget should draw itself inside that region of space. Layout region space allocated to widgets
/// is then tracked.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    x: f32,                       // marks start of free horizontal space in the region
    y: f32,                       // marks start of free vertical space in the rgion
    fill_w: bool,                 // fill width of layout
    fill_h: bool,                 // fill height of layout
    expand: bool,                 // layout expands to track all content allocated
    mode: LayoutMode,             // layout mode directive
    size: Vec2,                   // size of the layout region
    pos: Vec2,                    // position of the layout region
    spacing: f32,                 // space to include between widgets
    margin: RectOffset,           // push content in from layout edges this amount
    widgets: Vec<(String, Rect)>, // cache sizing and positioning allocations

    // Parent layout properties
    parent: Option<Rect>, // parent layout to position inside of
}

impl Layout {
    /// Create the default layout
    pub fn new() -> Self {
        Self::default()
    }

    /// Create the default root layout filling the entire screen
    pub fn root() -> Self {
        Self::default().with_size_f()
    }

    /// Create a horizontal layout
    pub fn horz() -> Self {
        Self::default().with_horz()
    }

    /// Create a vertical layout
    pub fn vert() -> Self {
        Self::default().with_vert()
    }

    /// Set horizontal layout mode
    pub fn with_horz(self) -> Self {
        Self { mode: LayoutMode::Horizontal, ..self }
    }

    /// Set vertical layout mode
    pub fn with_vert(self) -> Self {
        Self { mode: LayoutMode::Vertical, ..self }
    }

    /// Set the layout static position
    pub fn with_pos(self, x: f32, y: f32) -> Self {
        Self { pos: vec2(x, y), ..self }
    }

    /// Set the layout size to full screen
    pub fn with_size_f(self) -> Self {
        Self { size: screen(), ..self }
    }

    /// Set the layout size to a percentage
    /// * `width` is a percentage of the screen/parent width range of (0.01 - 1.0)
    /// * `height` is a percentage of the screen/parent height range of (0.01 - 1.0)
    pub fn with_size_p(self, width: f32, height: f32) -> Self {
        let parent = if let Some(parent) = self.parent { vec2(parent.w, parent.h) } else { screen() };
        Self { size: vec2(parent.x * width, parent.y * height), ..self }
    }

    /// Set the layout size to a static size
    pub fn with_size_s(self, width: f32, height: f32) -> Self {
        Self { size: vec2(width, height), ..self }
    }

    /// Fill the entire layout
    pub fn with_fill(self) -> Self {
        Self { fill_w: true, fill_h: true, ..self }
    }

    /// Fill the entire width of the layout
    pub fn with_fill_w(self) -> Self {
        Self { fill_w: true, ..self }
    }

    /// Fill the entire height of the layout
    pub fn with_fill_h(self) -> Self {
        Self { fill_h: true, ..self }
    }

    /// Configure layout expansion. When enabled disables fill properties
    pub fn with_expand(self) -> Self {
        Self { expand: true, fill_h: false, fill_w: false, ..self }
    }

    /// Space to allocate between widgets
    pub fn with_spacing(self, spacing: f32) -> Self {
        Self { spacing, ..self }
    }

    /// Allow for the given amount of space in the layout before the widget frame's are drawn.
    /// Layout's have margin while widgets have padding which is space inside the frame.
    pub fn with_margin(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { margin: RectOffset { left, right, top, bottom }, ..self }
    }

    /// Allow for the given amount of space in the layout before the widget frame's are drawn.
    /// Layout's have margins while widgets have padding which is space inside the frame.
    pub fn with_margin_p(self, padding: RectOffset) -> Self {
        Self { margin: padding, ..self }
    }

    /// Override the layout's position. This will cause a recalculation of all cached widget
    /// positions.
    pub fn set_pos(&mut self, x: f32, y: f32) {
        let widgets = self.widgets.clone();
        self.reset();
        self.pos = vec2(x, y);

        for (id, rect) in widgets.iter() {
            self.alloc(id, vec2(rect.w, rect.h));
        }
    }

    /// Size of the layout
    pub fn size(&self) -> Vec2 {
        self.size
    }

    /// Get cached size of the given widget
    pub fn size_of(&self, id: &str) -> Vec2 {
        match self.widgets.iter().find(|x| x.0 == id).map(|x| vec2(x.1.w, x.1.h)) {
            Some(x) => x,
            _ => Vec2::default(),
        }
    }

    /// Get cached size and position of the given widget
    pub fn widget_alloc(&self, id: &str) -> Option<&Rect> {
        self.widgets.iter().find(|x| x.0 == id).map(|x| &x.1)
    }

    /// Create a new layout inside the given layout
    /// * new layout size is the size of the current layout minus padding
    /// * new layout position is the position of the current layout after padding
    pub fn nest(&self) -> Self {
        let parent = Rect::new(
            self.pos.x + self.margin.left,
            self.pos.y + self.margin.top,
            self.size.x - self.margin.left - self.margin.right,
            self.size.y - self.margin.top - self.margin.bottom,
        );
        Self {
            size: vec2(parent.w, parent.h),
            pos: vec2(parent.x, parent.y),
            parent: Some(parent),
            ..Self::default()
        }
    }

    /// Reset the layout's tracking
    pub fn reset(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.size = Vec2::default();
        self.pos = Vec2::default();
        self.widgets.clear();
    }

    /// Allocate space for the given widget.
    /// * returns tuple (position, size)
    /// * TODO: need to determine how to handle allocations beyond the layout region
    pub fn alloc(&mut self, id: &str, size: Vec2) -> (Vec2, Vec2) {
        // Allocate space for the widget
        let mut rect = Rect::new(
            self.x + self.pos.x + self.margin.left,
            self.y + self.pos.y + self.margin.top,
            size.x,
            size.y,
        );

        // Handle fill width and height
        if self.fill_w {
            rect.w = self.size.x - self.margin.left - self.margin.right;
        }
        if self.fill_h {
            rect.h = self.size.y - self.margin.top - self.margin.bottom;
        }

        match self.mode {
            LayoutMode::Horizontal => {
                self.x += rect.w;
                if self.expand {
                    self.size.x += rect.w;
                    // Take the largest y value
                    if self.size.y < rect.h {
                        self.size.y = rect.h;
                    }
                }

                // Allocate spacing if not the first widget
                if !self.widgets.is_empty() {
                    rect.x += self.spacing;
                    self.x += self.spacing;
                    if self.expand {
                        self.size.x += self.spacing;
                    }
                }
            },
            LayoutMode::Vertical => {
                self.y += rect.h;
                if self.expand {
                    self.size.y += rect.h;
                }

                // Allocate spacing if not the first widget
                if !self.widgets.is_empty() {
                    rect.y += self.spacing;
                    self.y += self.spacing;
                    if self.expand {
                        self.size.y += self.spacing;
                    }
                }
            },
        }

        // Track the widget space allocation
        self.widgets.push((id.to_string(), rect));

        (vec2(rect.x, rect.y), vec2(rect.w, rect.h))
    }
}

/// Define different layout modes
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutMode {
    /// Stack widgets and containers horizontally
    Horizontal,

    /// Stack widgets and containers vertically
    Vertical,
}

impl Default for LayoutMode {
    fn default() -> Self {
        LayoutMode::Horizontal
    }
}
