use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Region {
    size: Vec2,
    color: Color,
}

impl Region {
    pub fn new(color: Color) -> Self {
        Self { size: vec2(50., 50.), color }
    }

    pub fn show(&mut self, ui: &mut Ui, layout: &mut Layout) {
        let (pos, size) = layout.alloc(self.size);
        draw_rectangle(pos.x, pos.y, size.x, size.y, self.color);
    }
}

/// Layout describes a region of space and provides mechanisms for calculating where and how a
/// widget should draw itself inside that region of space.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    x: f32,             // marks start of free horizontal space in the region
    y: f32,             // marks start of free vertical space in the rgion
    fill_w: bool,       // fill width of layout
    fill_h: bool,       // fill height of layout
    mode: LayoutMode,   // layout mode directive
    margin: RectOffset, // space to reserve outside frame
    size: Vec2,         // size of the layout region
    pos: Vec2,          // position of the layout region
    spacing: f32,       // space to include between widgets
    objects: Vec<Rect>, // track allocated space in the region

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

    /// Set the layout size to full screen
    pub fn size_f(self) -> Self {
        Self { size: screen(), ..self }
    }

    /// Set the layout size to a percentage
    /// * `width` is a percentage of the screen width range of (0.01 - 1.0)
    /// * `height` is a percentage of the screen height range of (0.01 - 1.0)
    pub fn size_p(self, width: f32, height: f32) -> Self {
        Self { size: vec2(screen_width() * width, screen_height() * height), ..self }
    }

    /// Set the layout size to a static size
    pub fn size_s(self, width: f32, height: f32) -> Self {
        Self { size: vec2(width, height), ..self }
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

    /// Adjust position to allow for the given margin outside this layout
    pub fn margin(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { x: self.x + left, y: self.y + top, margin: RectOffset { left, right, top, bottom }, ..self }
    }

    /// Create a new layout inside the given layout
    pub fn nest(&self) -> Self {
        Self {
            parent: Some(Rect::new(
                self.pos.x + self.margin.left,
                self.pos.y + self.margin.top,
                self.size.x,
                self.size.y,
            )),
            ..Self::default()
        }
    }

    /// Allocate space for the given widget.
    /// * returns tuple (position, size)
    /// * TODO: need to determine how to handle allocations behind the layout region
    pub fn alloc(&mut self, size: Vec2) -> (Vec2, Vec2) {
        // Allocate space for the widget
        let mut rect = Rect::new(self.x, self.y, size.x, size.y);

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
