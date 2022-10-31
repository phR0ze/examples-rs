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
//! ## Spacing
//! When packing widgets in a layout consecutively spacing can be applied to provide a consistent
//! space between widgets
use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};

/// SharedLayout defines a sharable interior mutable Layout object
type SharedLayout = Rc<RefCell<LayoutInner>>;

// Internal implemenation detail for sharing ownership of layouts
#[derive(Clone, Debug, PartialEq)]
struct LayoutInner {
    id: String,                   // layout identifier
    dirty: bool,                  // track if the layout's size or position need recalculated
    size: Vec2,                   // size of the layout region excluding margins
    offset: Vec2,                 // positional offset inside parent i.e. not absolute coordinates
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
            dirty: true, // always dirty by default
            size: Vec2::default(),
            offset: Vec2::default(),
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
        debug!("align: {}", &self.id);
        let parent_mode = self.parent.as_ref().map(|x| x.borrow().mode).unwrap_or(PackMode::default());
        let parent_spacing = self.parent.as_ref().map(|x| x.borrow().spacing).unwrap_or(0.);
        let parent_idx = self.parent.as_ref().and_then(|x| x.borrow().index(&self.id)).unwrap_or(0) as i32;
        let parent_len = self.parent.as_ref().and_then(|x| Some(x.borrow().layouts.len())).unwrap_or(0) as i32;

        let mut pos = self.align.relative(self.size, parent_size, parent_pos);
        pos = match parent_mode {
            PackMode::LeftToRight => vec2(parent_pos.x + self.offset.x, pos.y),
            PackMode::TopToBottom => vec2(pos.x, parent_pos.y + self.offset.y),
            PackMode::Align => pos,
        };

        // Handle spacing
        if parent_idx != parent_len - 1 {
            if let PackMode::LeftToRight = parent_mode {
                pos.x += parent_spacing * parent_idx as f32;
            } else if let PackMode::TopToBottom = parent_mode {
                pos.y += parent_spacing * parent_idx as f32;
            }
        }

        // Handle margins
        pos.x += self.margins.left;
        pos.y += self.margins.top;

        pos
    }

    /// Get sub-layout's index in this layout
    fn index(&self, id: &str) -> Option<usize> {
        self.layouts.iter().position(|x| x.borrow().id == id)
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

    /// Create a horizontal layout
    pub fn horz<T: AsRef<str>>(id: T) -> Self {
        let layout = Self::new(id);
        {
            let inner = &mut *layout.0.borrow_mut();
            inner.dirty = true;
            inner.mode = PackMode::LeftToRight;
        }
        layout
    }

    /// Create a vertical layout
    pub fn vert<T: AsRef<str>>(id: T) -> Self {
        let layout = Self::new(id);
        {
            let inner = &mut *layout.0.borrow_mut();
            inner.dirty = true;
            inner.mode = PackMode::TopToBottom;
        }
        layout
    }

    /// Set layout alignment
    pub fn align(self, align: Align) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.align = align;
        }
        self
    }

    /// Set the layout size to full screen
    /// * disables layout expansion
    pub fn size_f(self) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
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
    pub fn size_p(self, width: f32, height: f32) -> Self {
        {
            let size = if let Some(parent) = &self.0.borrow().parent { parent.borrow().size } else { screen() };
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.expand = false;
            layout.size = vec2(size.x * width, size.y * height);
        }
        self
    }

    /// Set the layout size to a static size
    /// * disables layout expansion
    pub fn size_s(self, width: f32, height: f32) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.expand = false;
            layout.size = vec2(width, height);
        }
        self
    }

    /// Fill the entire layout
    pub fn fill(self) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.fill_w = true;
            layout.fill_h = true;
        }
        self
    }

    /// Fill the entire width of the layout
    pub fn fill_w(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = true;
        }
        self
    }

    /// Fill the entire height of the layout
    pub fn fill_h(self) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.fill_h = true;
        }
        self
    }

    /// Configure layout expansion
    /// * When enabled disables fill properties
    pub fn expand(self) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.expand = true;
            layout.fill_w = false;
            layout.fill_h = false;
        }
        self
    }

    /// Space to allocate between widgets
    pub fn spacing(self, spacing: f32) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.spacing = spacing;
        }
        self
    }

    /// Space reserved outside the boundaries of the layout
    pub fn margins(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.margins = RectOffset { left, right, top, bottom };
        }
        self
    }

    /// Space reserved outside the boundaries of the layout
    pub fn margins_p(self, margins: RectOffset) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.margins = margins;
        }
        self
    }

    /// Add a parent layout for relative alignment
    /// * when align is set the LayoutMode won't take affect
    pub fn parent(self, parent: Layout) -> Self {
        {
            let layout = &mut *self.0.borrow_mut();
            layout.dirty = true;
            layout.parent = Some(parent.0.clone());
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

    /// Set the layout's size
    pub fn set_size_s(&self, size: Vec2) {
        let layout = &mut *self.0.borrow_mut();
        layout.dirty = true;
        layout.expand = false;
        layout.size = size;
    }

    /// Set sub-layout by id
    pub fn set_sub(&self, sub_layout: Layout) {
        let layout = &mut *self.0.borrow_mut();
        if let Some(i) = layout.index(&sub_layout.id()) {
            layout.layouts[i] = sub_layout.0.clone();
        } else {
            layout.layouts.push(sub_layout.0.clone());
        }
        layout.dirty = true;
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

    // Create a new layout inside this layout
    fn alloc<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let mut sub_layout = Layout::new(id.as_ref().to_string()).parent(Layout(self.0.clone()));
        if let Some(size) = size {
            sub_layout = sub_layout.size_s(size.x, size.y).expand();
        } else {
            sub_layout = sub_layout.expand();
        }
        sub_layout
    }

    /// Create a new sub-layout inside this layout
    /// * Adds the new sub-layout to the end of the sub-layout list
    pub fn alloc_append<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let sub_layout = self.alloc(id.as_ref(), size);
        let layout = &mut *self.0.borrow_mut();
        layout.layouts.push(sub_layout.0.clone());
        layout.dirty = true;
        sub_layout
    }

    /// Create a new sub-layout inside this layout
    /// * Adds the new sub-layout to the begining of the sub-layout list
    pub fn alloc_prepend<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let sub_layout = self.alloc(id.as_ref(), size);
        let layout = &mut *self.0.borrow_mut();
        layout.layouts.insert(0, sub_layout.0.clone());
        layout.dirty = true;
        sub_layout
    }

    /// Append the given sub-layout to this layout
    /// * Adds the new sub-layout to the end of the sub-layout list if it doesn't already exist
    pub fn append(&self, sub_layout: &Layout) {
        if self.sub_idx(&sub_layout.id()).is_none() {
            {
                sub_layout.0.borrow_mut().parent = Some(self.0.clone());
                let layout = &mut *self.0.borrow_mut();
                layout.layouts.push(sub_layout.0.clone());
                layout.dirty = true;
            }
            self.update();
        }
    }

    /// Calculate and set the size and positional offset of the layout and sub-layouts
    /// * only performs calculation if needed
    /// * takes into account margins
    /// * size update has no effect unless expansion is set
    pub fn update(&self) {
        let layout = &mut *self.0.borrow_mut();

        // Bail if no update is needed
        if !layout.dirty {
            return;
        }

        // Calculate layout size and set positional offsets along the way
        let mut cursor = Vec2::default(); // track where to start drawing widget inside parent
        let mut size = Vec2::default();
        for x in layout.layouts.iter_mut() {
            let sub = &mut *x.borrow_mut();
            debug!("Processing: {}", &sub.id);

            // Update the sub-layout's positional offset
            sub.offset = cursor;

            // Caculate the sub-layout's size
            let mut sub_width = sub.size.x + sub.margins.left + sub.margins.right;
            let mut sub_height = sub.size.y + sub.margins.top + sub.margins.bottom;

            // Take fill directives into account
            if layout.fill_w && !layout.expand {
                sub_width = layout.size.x;
            }
            if layout.fill_h && !layout.expand {
                sub_height = layout.size.y;
            }

            match layout.mode {
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
        if layout.expand {
            layout.size = size;
        }
        layout.dirty = false;
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
        PackMode::Align
    }
}
