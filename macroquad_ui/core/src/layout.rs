//! Layout describes a region of space in which widgets should be drawn and provides mechanisms for
//! calculating and tracking where and how they should be drawn.
//!
//! ## Defaults
//! * horizontal layout
//! * expansion enabled
//!
//! ## Layout mode
//! Layout modes provide different terpretations of how widgets should be packed into the layout's
//! defined region of space. The default horizontal layout will add widgets by default from left to
//! right while the vertical layout will add widgets by default from top to bottom.
//!
//! ## Align directive
//! The alignment directive as modified by the various align functions is used to guide the
//! calculation of the widgets position in its parent layout.
//!
//! ## Expand directive
//! Layout expansion is the default mode. In this mode the layout will expand its size to account
//! for all content allocations. This is very useful for cases where you don't know the size of the
//! layout in advance and need to build that knowledge based on the widgets it is composed of taking
//! into account margins and/or alignment preferences for one or more widgts. For example Button is
//! composed of an Icon, Label and a Frame each of which require layout mangement.
//!
//! ## Fill directive
//! The fill properties `fill_w`, `fill_h` and `fill` direct the layout to have the allocated widget
//! fill the (w) width, (h) height or both directions. This provides the ability to create a Panel
//! to be used as a menu with a fixed size and then have buttons of unknown size fill the width of
//! the menu with margins taken into acocunt.
//!
//! ## Position updates
//! When updating a layout's position the layouts within that layout will have their positions
//! updated to match their parent's relative position.
use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};

/// Convenience type for shared interior mutable inner Layout object
pub type SharedLayout = Rc<RefCell<LayoutInner>>;

// Internal implemenation detail for sharing ownership of layouts
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutInner {
    id: String,          // layout identifier
    x: f32,              // marks start of free horizontal space in the region
    y: f32,              // marks start of free vertical space in the rgion
    pos: Vec2,           // position of the layout region excluding margins
    size: Vec2,          // size of the layout region excluding margins
    fill_w: bool,        // fill width of layout
    fill_h: bool,        // fill height of layout
    expand: bool,        // layout expands to track all content allocated
    align: Align,        // alignment in the parent layout
    mode: LayoutMode,    // layout mode directive
    spacing: f32,        // space to include between widgets
    margins: RectOffset, // space outside the frame edge
}

/// Layout describes a region of space and provides mechanisms for calculating where and how a
/// widget should draw itself inside that region of space. Layout region space allocated to widgets
/// is then tracked.
#[derive(Clone, Debug, PartialEq)]
pub struct Layout {
    layout: SharedLayout,         // layout
    layouts: Vec<Layout>,         // child layouts
    parent: Option<SharedLayout>, // parent layout
}

impl Layout {
    /// Create the default layout
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self {
            layout: Rc::new(RefCell::new(LayoutInner {
                id: id.as_ref().to_string(),
                x: 0.,
                y: 0.,
                pos: Vec2::default(),
                size: Vec2::default(),
                fill_w: false,
                fill_h: false,
                expand: true, // enable expansion by default
                mode: LayoutMode::default(),
                align: Align::default(),
                spacing: 0.,
                margins: RectOffset::default(),
            })),
            layouts: vec![],
            parent: Option::<SharedLayout>::default(),
        }
    }

    /// Create the default root layout filling the entire screen
    pub fn root<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).with_size_f()
    }

    /// Create a horizontal layout
    pub fn horz<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).with_horz()
    }

    /// Create a vertical layout
    pub fn vert<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).with_vert()
    }

    /// Set the layout's id
    pub fn with_id<T: AsRef<str>>(self, id: T) -> Self {
        self.layout.borrow_mut().id = id.as_ref().to_string();
        self
    }

    /// Set horizontal layout mode
    pub fn with_horz(self) -> Self {
        self.layout.borrow_mut().mode = LayoutMode::Horizontal;
        self
    }

    /// Set vertical layout mode
    pub fn with_vert(self) -> Self {
        self.layout.borrow_mut().mode = LayoutMode::Vertical;
        self
    }

    /// Set the layout static position
    pub fn with_pos(self, x: f32, y: f32) -> Self {
        self.layout.borrow_mut().pos = vec2(x, y);
        self
    }

    /// Set the layout size to full screen
    /// * disables layout expansion
    pub fn with_size_f(self) -> Self {
        {
            let layout = &mut *self.layout.borrow_mut();
            layout.expand = false;
            layout.size = screen();
        }
        self
    }

    /// Set the layout size to a percentage of the parent layout
    /// * disables layout expansion
    /// * parent defaults to full screen if not set
    /// * `width` is a percentage of the screen/parent width range of (0.01 - 1.0)
    /// * `height` is a percentage of the screen/parent height range of (0.01 - 1.0)
    pub fn with_size_p(self, width: f32, height: f32) -> Self {
        {
            let size = if let Some(parent) = &self.parent { parent.borrow().size } else { screen() };
            let layout = &mut *self.layout.borrow_mut();
            layout.expand = false;
            layout.size = vec2(size.x * width, size.y * height);
        }
        self
    }

    /// Set the layout size to a static size
    /// * disables layout expansion
    pub fn with_size_s(self, width: f32, height: f32) -> Self {
        {
            let layout = &mut *self.layout.borrow_mut();
            layout.expand = false;
            layout.size = vec2(width, height);
        }
        self
    }

    /// Fill the entire layout
    pub fn with_fill(self) -> Self {
        {
            let layout = &mut *self.layout.borrow_mut();
            layout.fill_w = true;
            layout.fill_h = true;
        }
        self
    }

    /// Fill the entire width of the layout
    pub fn with_fill_w(self) -> Self {
        {
            let layout = &mut *self.layout.borrow_mut();
            layout.fill_w = true;
        }
        self
    }

    /// Fill the entire height of the layout
    pub fn with_fill_h(self) -> Self {
        {
            let layout = &mut *self.layout.borrow_mut();
            layout.fill_h = true;
        }
        self
    }

    /// Configure layout expansion
    /// * When enabled disables fill properties
    pub fn with_expand(self) -> Self {
        {
            let layout = &mut *self.layout.borrow_mut();
            layout.expand = true;
            layout.fill_w = false;
            layout.fill_h = false;
        }
        self
    }

    /// Space to allocate between widgets
    pub fn with_spacing(self, spacing: f32) -> Self {
        self.layout.borrow_mut().spacing = spacing;
        self
    }

    /// Space reserved outside the boundaries of the layout
    pub fn with_margins(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        self.layout.borrow_mut().margins = RectOffset { left, right, top, bottom };
        self
    }

    /// Space reserved outside the boundaries of the layout
    pub fn with_margins_p(self, margins: RectOffset) -> Self {
        self.layout.borrow_mut().margins = margins;
        self
    }

    /// Add a parent layout for relative alignment
    pub fn with_parent(self, parent: SharedLayout) -> Self {
        Self { parent: Some(parent), ..self }
    }

    /// Get the layout's current margins
    pub fn get_margins(&self) -> RectOffset {
        self.layout.borrow().margins
    }

    /// Get sub-layout by id
    pub fn get_layout(&self, id: &str) -> Option<&Layout> {
        self.layouts.iter().find(|x| x.layout.borrow().id == id)
    }

    /// Get mutable sub-layout by id
    pub fn get_layout_mut(&mut self, id: &str) -> Option<&mut Layout> {
        self.layouts.iter_mut().find(|x| x.layout.borrow().id == id)
    }

    /// Get the sub-layout's content position and size by id
    /// * position accounts for margins
    /// * size accounts for margins
    /// * returns (pos, size)
    pub fn get_layout_shape(&self, id: &str) -> Option<(Vec2, Vec2)> {
        self.get_layout(id).map(|x| x.get_shape())
    }

    /// Get the layout's position and size
    /// * position accounts for margins
    /// * size accounts for margins
    /// * returns (pos, size)
    pub fn get_shape(&self) -> (Vec2, Vec2) {
        (self.get_pos(), self.get_size())
    }

    /// Get the layout's content size
    pub fn get_size(&self) -> Vec2 {
        self.layout.borrow().size
    }

    /// Get the layout's content position i.e. position + padding
    pub fn get_pos(&self) -> Vec2 {
        self.layout.borrow().pos
    }

    /// Set the layout's id
    pub fn set_id<T: AsRef<str>>(&self, id: T) {
        self.layout.borrow_mut().id = id.as_ref().to_string();
    }

    /// Set the layout's size
    pub fn set_size_s(&self, size: Vec2) {
        self.layout.borrow_mut().size = size;
    }

    /// Set the sub-layout's size by id
    /// * same as `set_layout_size_p` but takes float params instead of Vec2
    pub fn set_layout_size_s(&mut self, id: &str, width: f32, height: f32) {
        self.get_layout_mut(id).map(|x| x.set_size_s(vec2(width, height)));
    }

    /// Set the sub-layout's size by id
    /// * same as `set_layout_size_s` but takes Vec2 object instead of floats
    pub fn set_layout_size_p(&mut self, id: &str, size: Vec2) {
        self.get_layout_mut(id).map(|x| x.set_size_s(size));
    }

    /// Set sub-layout by id
    pub fn set_layout(&mut self, layout: Layout) {
        if let Some(i) = self.layouts.iter().position(|x| x.layout.borrow().id == layout.layout.borrow().id) {
            self.layouts[i] = layout;
        } else {
            self.layouts.push(layout);
        }
    }

    /// Set the size of the layout based on a calculation of total sub-layout size
    pub fn calc_size(&mut self) {
        let inner = &mut *self.layout.borrow_mut();

        inner.x = 0.;
        inner.y = 0.;
        for x in self.layouts.iter() {
            let layout = &*x.layout.borrow();
            let layout_width = layout.size.x + layout.margins.left + layout.margins.right;
            let layout_height = layout.size.y + layout.margins.top + layout.margins.bottom;
            match inner.mode {
                LayoutMode::Horizontal => {
                    inner.x += layout_width;
                    if inner.y < layout_height {
                        inner.y = layout_height;
                    }
                },
                LayoutMode::Vertical => {
                    if inner.x < layout_width {
                        inner.x = layout_width;
                    }
                    inner.y += layout_height;
                },
            }
        }
        inner.size = vec2(inner.x, inner.y);
    }

    /// Create a new layout inside this layout
    pub fn alloc<T: AsRef<str>>(&mut self, id: T, size: Option<Vec2>) -> &Layout {
        let mut layout = Layout::new(id.as_ref().to_string());

        // Set parent if it exists
        if let Some(parent) = &self.parent {
            layout = layout.with_parent(parent.clone());
        }

        // If size is given set and re-set expand
        if let Some(size) = size {
            layout = layout.with_size_s(size.x, size.y).with_expand();
        }

        // let mut rect = Rect::new(
        //     self.x + self.pos.x + self.margin.left,
        //     self.y + self.pos.y + self.margin.top,
        //     size.x,
        //     size.y,
        // );

        // // Handle fill width and height
        // if self.fill_w {
        //     rect.w = self.size.x - self.margin.left - self.margin.right;
        // }
        // if self.fill_h {
        //     rect.h = self.size.y - self.margin.top - self.margin.bottom;
        // }

        // match self.mode {
        //     LayoutMode::Horizontal => {
        //         self.x += rect.w;
        //         if self.expand {
        //             self.size.x += rect.w;
        //             // Take the largest y value
        //             if self.size.y < rect.h {
        //                 self.size.y = rect.h;
        //             }
        //         }

        //         // Allocate spacing if not the first widget
        //         if !self.layouts.is_empty() {
        //             rect.x += self.spacing;
        //             self.x += self.spacing;
        //             if self.expand {
        //                 self.size.x += self.spacing;
        //             }
        //         }
        //     },
        //     LayoutMode::Vertical => {
        //         self.y += rect.h;
        //         if self.expand {
        //             self.size.y += rect.h;
        //         }

        //         // Allocate spacing if not the first widget
        //         if !self.layouts.is_empty() {
        //             rect.y += self.spacing;
        //             self.y += self.spacing;
        //             if self.expand {
        //                 self.size.y += self.spacing;
        //             }
        //         }
        //     },
        // }

        // Track the layout allocation
        self.layouts.push(layout);
        self.get_layout(id.as_ref()).unwrap()
    }
}

/// Align is a directive used to guide the calculation of the widgets position in its parent layout
#[derive(Clone, Debug, PartialEq)]
pub enum Align {
    /// Align widget in the center horizontally and in the top vertically
    /// * accepts an optional offset value
    CenterTop(Option<RectOffset>),

    /// Align in the center horizontally and in the center vertically
    /// * accepts an optional offset value
    Center(Option<RectOffset>),

    /// Align in the center horizontally and in the bottom vertically
    /// * accepts an optional offset value
    CenterBottom(Option<RectOffset>),

    /// Align in the right horizontally and in the top vertically
    /// * accepts an optional offset value
    RightTop(Option<RectOffset>),

    /// Align in the right horizontally and in the center vertically
    /// * accepts an optional offset value
    RightCenter(Option<RectOffset>),

    /// Align in the right horizontally and in the bottom vertically
    /// * accepts an optional offset value
    RightBottom(Option<RectOffset>),

    /// Align in the left horizontally and in the top vertically
    /// * accepts an optional offset value
    LeftTop(Option<RectOffset>),

    /// Align in the left horizontally and in the center vertically
    /// * accepts an optional offset value
    LeftCenter(Option<RectOffset>),

    /// Align in the left horizontally and in the bottom vertically
    /// * accepts an optional offset value
    LeftBottom(Option<RectOffset>),

    /// Align horizontally with the given value and vertically with the given value
    Static(f32, f32),
}

impl Default for Align {
    fn default() -> Self {
        Align::Center(None)
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
