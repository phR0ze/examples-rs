use crate::prelude::*;

/// Layout describes a region of space and provides mechanisms for calculating where and how a
/// widget should draw itself inside that region of space.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    x: f32,              // marks start of free horizontal space in the region
    y: f32,              // marks start of free vertical space in the rgion
    fill_w: bool,        // fill width of layout
    fill_h: bool,        // fill height of layout
    mode: LayoutMode,    // layout mode directive
    size: Vec2,          // size of the layout region
    pos: Vec2,           // position of the layout region
    spacing: f32,        // space to include between widgets
    padding: RectOffset, // push content in from layout edges this amount
    objects: Vec<Rect>,  // track allocated space in the region

    // Parent layout properties
    parent: Option<Rect>, // parent layout to position inside of
}

impl Layout {
    /// Create the default root layout filling the entire screen
    pub fn root() -> Self {
        Self::default().size_f()
    }

    /// Create a horizontal layout
    pub fn horz() -> Self {
        Self::default().horz_m()
    }

    /// Create a vertical layout
    pub fn vert() -> Self {
        Self::default().vert_m()
    }

    /// Set horizontal layout mode
    pub fn horz_m(self) -> Self {
        Self { mode: LayoutMode::Horizontal, ..self }
    }

    /// Set vertical layout mode
    pub fn vert_m(self) -> Self {
        Self { mode: LayoutMode::Vertical, ..self }
    }

    /// Set the layout static position
    pub fn pos_s(self, x: f32, y: f32) -> Self {
        Self { pos: vec2(x, y), ..self }
    }

    /// Set the layout size to full screen
    pub fn size_f(self) -> Self {
        Self { size: screen(), ..self }
    }

    /// Set the layout size to a percentage
    /// * `width` is a percentage of the screen/parent width range of (0.01 - 1.0)
    /// * `height` is a percentage of the screen/parent height range of (0.01 - 1.0)
    pub fn size_p(self, width: f32, height: f32) -> Self {
        let parent = if let Some(parent) = self.parent { vec2(parent.w, parent.h) } else { screen() };
        Self { size: vec2(parent.x * width, parent.y * height), ..self }
    }

    /// Set the layout size to a static size
    pub fn size_s(self, width: f32, height: f32) -> Self {
        Self { size: vec2(width, height), ..self }
    }

    /// Fill the entire layout
    pub fn fill(self) -> Self {
        Self { fill_w: true, fill_h: true, ..self }
    }

    /// Fill the entire width of the layout
    pub fn fill_w(self) -> Self {
        Self { fill_w: true, ..self }
    }

    /// Fill the entire height of the layout
    pub fn fill_h(self) -> Self {
        Self { fill_h: true, ..self }
    }

    /// Space to allocate between widgets
    pub fn spacing(self, spacing: f32) -> Self {
        Self { spacing, ..self }
    }

    /// Push content in from edges of layout this amount
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { padding: RectOffset { left, right, top, bottom }, ..self }
    }

    /// Create a new layout inside the given layout
    /// * new layout size is the size of the current layout minus padding
    /// * new layout position is the position of the current layout after padding
    pub fn nest(&self) -> Self {
        let parent = Rect::new(
            self.pos.x + self.padding.left,
            self.pos.y + self.padding.top,
            self.size.x - self.padding.left - self.padding.right,
            self.size.y - self.padding.top - self.padding.bottom,
        );
        Self {
            size: vec2(parent.w, parent.h),
            pos: vec2(parent.x, parent.y),
            parent: Some(parent),
            ..Self::default()
        }
    }

    /// Allocate space for the given widget.
    /// * returns tuple (position, size)
    /// * TODO: need to determine how to handle allocations behind the layout region
    pub fn alloc(&mut self, size: Vec2) -> (Vec2, Vec2) {
        // Allocate space for the widget
        let mut rect = Rect::new(
            self.x + self.pos.x + self.padding.left,
            self.y + self.pos.y + self.padding.top,
            size.x,
            size.y,
        );

        // Handle fill width and height
        if self.fill_w {
            rect.w = self.size.x - self.padding.left - self.padding.right;
        }
        if self.fill_h {
            rect.h = self.size.y - self.padding.top - self.padding.bottom;
        }

        match self.mode {
            LayoutMode::Horizontal => {
                self.x += rect.w;

                // Allocate spacing if not the first widget
                if !self.objects.is_empty() {
                    rect.x += self.spacing;
                    self.x += self.spacing;
                }
            },
            LayoutMode::Vertical => {
                self.y += rect.h;

                // Allocate spacing if not the first widget
                if !self.objects.is_empty() {
                    rect.y += self.spacing;
                    self.y += self.spacing;
                }
            },
        }

        // Track the widget space allocation
        self.objects.push(rect);

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
