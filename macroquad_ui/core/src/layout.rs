//! Layout describes a region of space in which widgets should be drawn and provides mechanisms for
//! calculating and tracking where and how they should be drawn.
//!
//! ## Defaults
//! * horizontal layout
//! * expansion enabled
//!
//! ## Pack mode
//! Pack modes provide different interpretations of how widgets should be packed into the layout's
//! defined region of space. The default LeftToRight mode will add widgets horizontally from left to
//! right while the TopToBottom layout will add widgets vertically from top to bottom. The alignment
//! directive can be combined with the pack mode to provide centering for the uncontrolled
//! direction. For example a
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

/// SharedLayout defines a sharable interior mutable Layout object
type SharedLayout = Rc<RefCell<LayoutInner>>;

// Internal implemenation detail for sharing ownership of layouts
#[derive(Clone, Debug, PartialEq)]
struct LayoutInner {
    id: String,                   // layout identifier
    pos: Vec2,                    // positional offset inside parent i.e. not absolute coordinates
    size: Vec2,                   // size of the layout region excluding margins
    dirty: bool,                  // track if the layout's size or position need recalculated
    fill_w: bool,                 // fill width of layout
    fill_h: bool,                 // fill height of layout
    expand: bool,                 // layout expands to track all content allocated
    align: Align,                 // alignment in the parent layout
    mode: PackMode,               // layout mode directive
    spacing: f32,                 // space to include between widgets
    margins: RectOffset,          // space outside the frame edge
    layouts: Vec<SharedLayout>,   // sub-layouts
    parent: Option<SharedLayout>, // parent layout
}

impl LayoutInner {
    fn new<T: AsRef<str>>(id: T) -> SharedLayout {
        Rc::new(RefCell::new(Self {
            id: id.as_ref().to_string(),
            pos: Vec2::default(),
            size: Vec2::default(),
            dirty: true, // always dirty by default
            fill_w: false,
            fill_h: false,
            expand: true, // enable expansion by default
            mode: PackMode::default(),
            align: Align::default(),
            spacing: 0.,
            margins: RectOffset::default(),
            layouts: vec![],
            parent: Option::<SharedLayout>::default(),
        }))
    }

    // Calculate the widgets alignment based on its
    // * size, positional offset, margins and mode
    // * parent's size and positional offset
    fn align(&self, parent_pos: Vec2, parent_size: Vec2) -> Vec2 {
        let mut pos = self.align.relative(self.size, parent_size, parent_pos);
        pos = match self.mode {
            PackMode::LeftToRight => vec2(parent_pos.x + self.pos.x, pos.y),
            PackMode::TopToBottom => vec2(pos.x, parent_pos.y + self.pos.y),
            PackMode::Align => pos,
        };

        // Handle margins
        pos.x += self.margins.left;
        pos.y += self.margins.top;

        pos
    }

    // Get parent layout's position and size
    // * assumes parent's parent size and positional offset are already updated
    // * position includes margins
    // * returns (pos, size)
    fn parent_shape(&self) -> (Vec2, Vec2) {
        let size = match &self.parent {
            Some(parent) => parent.borrow().size,
            _ => screen(), // default parent to full screen
        };

        let pos = match &self.parent {
            Some(parent) => {
                let (parent_pos, parent_size) = parent.borrow().parent_shape();
                let inner = parent.borrow();
                inner.align(parent_pos, parent_size)
            },
            _ => Vec2::default(),
        };

        (pos, size)
    }

    // Get the layout's position and size
    // * assumes layout size and position and parent size and positon are already updated
    // * position includes margins
    // * returns (pos, size)
    fn shape(&self) -> (Vec2, Vec2) {
        let (parent_pos, parent_size) = self.parent_shape();
        (self.align(parent_pos, parent_size), self.size)
    }
}

/// Layout describes a region of space and provides mechanisms for calculating where and how a
/// widget should draw itself inside that region of space. Layout region space allocated to widgets
/// is then tracked.
#[derive(Clone, Debug, PartialEq)]
pub struct Layout(SharedLayout);

// Layout constructors and builder functions
impl Layout {
    /// Create the default layout
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self(LayoutInner::new(id))
    }

    /// Create the default root layout filling the entire screen
    pub fn root<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).with_size_f()
    }

    /// Create a new layout as percentage of the screen
    pub fn percent<T: AsRef<str>>(id: T, width: f32, height: f32) -> Self {
        Self::new(id).with_size_p(width, height)
    }

    /// Create a horizontal layout
    pub fn horz<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).with_horz()
    }

    /// Create a vertical layout
    pub fn vert<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).with_vert()
    }

    /// Set layout alignment
    pub fn with_align(self, align: Align) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.align = align;
        }
        self
    }

    /// Set horizontal layout mode
    pub fn with_horz(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.mode = PackMode::LeftToRight;
        }
        self
    }

    /// Set vertical layout mode
    pub fn with_vert(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.mode = PackMode::TopToBottom;
        }
        self
    }

    /// Set the layout size to full screen
    /// * disables layout expansion
    pub fn with_size_f(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.expand = false;
            inner.size = screen();
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
            let size = if let Some(parent) = &self.0.borrow().parent { parent.borrow().size } else { screen() };
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.expand = false;
            inner.size = vec2(size.x * width, size.y * height);
        }
        self
    }

    /// Set the layout size to a static size
    /// * disables layout expansion
    pub fn with_size_s(self, width: f32, height: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.expand = false;
            inner.size = vec2(width, height);
        }
        self
    }

    /// Fill the entire layout
    pub fn with_fill(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = true;
            inner.fill_h = true;
        }
        self
    }

    /// Fill the entire width of the layout
    pub fn with_fill_w(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = true;
        }
        self
    }

    /// Fill the entire height of the layout
    pub fn with_fill_h(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_h = true;
        }
        self
    }

    /// Configure layout expansion
    /// * When enabled disables fill properties
    pub fn with_expand(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.expand = true;
            inner.fill_w = false;
            inner.fill_h = false;
        }
        self
    }

    /// Space to allocate between widgets
    pub fn with_spacing(self, spacing: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.spacing = spacing;
        }
        self
    }

    /// Space reserved outside the boundaries of the layout
    pub fn with_margins(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.margins = RectOffset { left, right, top, bottom };
        }
        self
    }

    /// Space reserved outside the boundaries of the layout
    pub fn with_margins_p(self, margins: RectOffset) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.margins = margins;
        }
        self
    }

    /// Add a parent layout for relative alignment
    /// * when align is set the LayoutMode won't take affect
    pub fn with_parent(self, parent: Layout) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.parent = Some(parent.0.clone());
        }
        self
    }
}

// Layout getters and helper functions
impl Layout {
    /// Get layout id
    pub fn id(&self) -> String {
        self.0.borrow().id.clone()
    }

    /// Get the layout's margins
    pub fn margins(&self) -> RectOffset {
        self.0.borrow().margins
    }

    /// Get parent layout's position and size
    /// * position accounts for margins
    /// * returns (pos, size)
    pub fn parent_shape(&self) -> (Vec2, Vec2) {
        self.0.borrow().parent_shape()
    }

    /// Get layout's position and size
    /// * position accounts for margins
    /// * returns (pos, size)
    pub fn shape(&self) -> (Vec2, Vec2) {
        self.0.borrow().shape()
    }

    /// Get sub-layout by id
    pub fn sub(&self, id: &str) -> Option<Layout> {
        self.0.borrow().layouts.iter().find(|x| x.borrow().id == id).map(|x| Layout(x.clone()))
    }

    /// Get sub-layout's index in this layout
    pub fn sub_idx(&self, id: &str) -> Option<usize> {
        self.0.borrow().layouts.iter().position(|x| x.borrow().id == id)
    }

    /// Get sub-layout's position and size by id
    /// * position accounts for margins
    /// * size accounts for margins
    /// * returns (pos, size)
    pub fn sub_shape(&self, id: &str) -> Option<(Vec2, Vec2)> {
        self.sub(id).map(|x| x.shape())
    }

    /// Set flag value for triggering a size and position update on next run
    pub fn set_dirty(&self, dirty: bool) {
        self.0.borrow_mut().dirty = dirty;
    }

    /// Set the layout's size
    pub fn set_size_s(&self, size: Vec2) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.expand = false;
        inner.size = size;
    }

    /// Set parent layout
    pub fn set_parent(&self, layout: &Layout) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.parent = Some(layout.0.clone());
    }

    /// Set sub-layout's size by id
    /// * same as `set_layout_size_p` but takes float params instead of Vec2
    pub fn set_sub_size_s(&self, id: &str, width: f32, height: f32) {
        self.sub(id).map(|x| x.set_size_s(vec2(width, height)));
    }

    /// Set the sub-layout's size by id
    /// * same as `set_layout_size_s` but takes Vec2 object instead of floats
    pub fn set_sub_size_p(&self, id: &str, size: Vec2) {
        self.sub(id).map(|x| x.set_size_s(size));
    }

    /// Set sub-layout by id
    pub fn set_sub(&self, layout: Layout) {
        layout.set_dirty(true);
        if let Some(i) = self.sub_idx(&layout.id()) {
            self.0.borrow_mut().layouts[i] = layout.0.clone();
        } else {
            self.0.borrow_mut().layouts.push(layout.0.clone());
        }
    }

    // Create a new layout inside this layout
    fn alloc_sub<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let mut layout = Layout::new(id.as_ref().to_string()).with_parent(Layout(self.0.clone()));
        if let Some(size) = size {
            layout = layout.with_size_s(size.x, size.y).with_expand();
        }
        layout
    }

    /// Create a new sub-layout inside this layout
    /// * Adds the new sub-layout to the end of the sub-layout list
    pub fn append_sub<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let layout = self.alloc_sub(id.as_ref(), size);
        self.0.borrow_mut().layouts.push(layout.0.clone());
        self.set_dirty(true);
        layout
    }

    /// Create a new sub-layout inside this layout
    /// * Adds the new sub-layout to the begining of the sub-layout list
    pub fn prepend_sub<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let layout = self.alloc_sub(id.as_ref(), size);
        self.0.borrow_mut().layouts.insert(0, layout.0.clone());
        self.set_dirty(true);
        layout
    }

    /// Calculate and set the size and positional offset of the layout and sub-layouts
    /// * only performs calculation if needed
    /// * takes into account margins
    /// * size update has no effect unless expansion is set
    pub fn update(&self) {
        let inner = &mut *self.0.borrow_mut();

        // Bail if no update is needed or expansion is not enabled
        if !inner.dirty || !inner.expand {
            return;
        }

        // Calculate layout size and set positional offsets along the way
        let mut cursor = Vec2::default(); // track where to start drawing widget inside parent
        let mut size = Vec2::default();
        for x in inner.layouts.iter_mut() {
            let sub = &mut *x.borrow_mut();

            // Update the sub-layout's positional offset
            sub.pos = cursor;

            // Caculate the sub-layout's size
            let mut sub_width = sub.size.x + sub.margins.left + sub.margins.right;
            let mut sub_height = sub.size.y + sub.margins.top + sub.margins.bottom;

            // Take fill directives into account
            if inner.fill_w && !inner.expand {
                sub_width = inner.size.x;
            }
            if inner.fill_h && !inner.expand {
                sub_height = inner.size.y;
            }

            match inner.mode {
                PackMode::LeftToRight | PackMode::Align => {
                    size.x += sub_width;
                    if size.y < sub_height {
                        size.y = sub_height;
                    }
                    cursor.x = size.x;
                },
                PackMode::TopToBottom => {
                    if size.x < sub_width {
                        size.x = sub_width;
                    }
                    size.y += sub_height;
                    cursor.y = size.y;
                },
            }
        }

        // Only update layout size based on sub-layout sums if in expand mode
        if !inner.expand {
            inner.size = size;
        }
    }
}

/// Define different layout modes
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PackMode {
    /// Pack widgets and containers horizontally
    LeftToRight,

    /// Pack widgets and containers vertically
    TopToBottom,

    /// Purely dependent on alignment for positioning
    Align,
}

impl Default for PackMode {
    fn default() -> Self {
        PackMode::LeftToRight
    }
}
