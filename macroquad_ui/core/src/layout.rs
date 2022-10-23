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
    region: Rect,       // region to constrain the layout to
    spacing: f32,       // space to include between widgets
    objects: Vec<Rect>, // track allocated space in the region
}

impl Layout {
    pub fn new(rect: Rect) -> Self {
        Self { region: rect, ..Self::default() }
    }

    /// Put layout in horizontal mode
    pub fn horz(self) -> Self {
        Self { mode: LayoutMode::Horizontal, ..self }
    }

    /// Put layout in vertical mode
    pub fn vert(self) -> Self {
        Self { mode: LayoutMode::Vertical, ..self }
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

    pub fn margin(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { x: self.x + left, y: self.y + top, margin: RectOffset { left, right, top, bottom }, ..self }
    }

    // Base layout is full screen and intended as the base for all Ui layout
    pub fn base() -> Self {
        let screen = screen();
        let mut layout = Self::default();
        layout.region = Rect::new(0., 0., screen.x, screen.y);
        layout
    }

    /// Allocate space for the given widget
    ///  * returns tuple (position, size)
    pub fn alloc(&mut self, size: Vec2) -> (Vec2, Vec2) {
        // Allocate space for the widget
        let mut rect = Rect::new(self.x, self.y, size.x, size.y);

        // Handle fill
        if self.fill_w {
            rect.w = self.region.w - self.margin.left - self.margin.right;
        }
        if self.fill_h {
            rect.h = self.region.h - self.margin.top - self.margin.bottom;
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
