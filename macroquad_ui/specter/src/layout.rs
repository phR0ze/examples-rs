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
//! calculation of the widgets position in its parent layout. Margins will still affect the position
//! even when an alignment directive existe.
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
//!
//! ## Margins
//! Margins are defined as additional space outside the widgets content area. Margins will affect
//! how the widget is drawn in two ways. The first way is that margins are included in the size of
//! the widget when considering how much space a widget will require inside a parent widget. However
//! margins don't affect the content area of the wiget. The second way margins affect the widget is
//! in its position. A left margin of 5.0 pts will positionally offset the widget to the right by
//! 5.0 pts. This means that having equal and opposite margins will cancel out the margin's
//! positional affect.
use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};

/// SharedLayout defines a sharable interior mutable Layout object
type SharedLayout = Rc<RefCell<LayoutInner>>;

// Internal implemenation detail for sharing ownership of layouts
#[derive(Clone, Debug, PartialEq)]
struct LayoutInner {
    // Internal only
    dirty: bool,  // track if the widget needs calculation updates
    offset: Vec2, // positional offset including margins

    // Exposed through Layout functions
    id: String,                   // layout identifier
    size: Vec2,                   // size of the layout region excluding margins
    fill_w: bool,                 // fill width of layout
    fill_h: bool,                 // fill height of layout
    expand: bool,                 // layout expands to track all content allocated
    align: Align,                 // alignment in the parent layout
    mode: Mode,                   // layout mode directive
    spacing: f32,                 // space to include between widgets
    margins: RectOffset,          // space outside the frame edge
    subs: Vec<SharedLayout>,      // sub-layouts
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
            mode: Mode::default(),
            align: Align::default(),
            spacing: 0.,
            margins: RectOffset::default(),
            subs: vec![],
            parent: Option::<SharedLayout>::default(),
        }))
    }

    // Calculate the widgets alignment based on its
    // * size, positional offset, margins and mode
    // * parent's size and positional offset
    fn pos(&self, parent_pos: Vec2, parent_size: Vec2) -> Vec2 {
        let (p_mode, p_spacing, p_idx, p_len) = match &self.parent {
            Some(parent) => {
                let inner = parent.borrow();
                (inner.mode, inner.spacing, inner.index(&self.id).unwrap_or(0) as f32, inner.subs.len())
            },
            _ => (Mode::default(), 0., 0., 0),
        };

        // Alignment against parent
        let mut pos = self.align.relative(self.size, parent_size, parent_pos);
        pos = match p_mode {
            // Override x coordinate using pre-calculation offset of size including margins
            Mode::LeftToRight => vec2(parent_pos.x + self.offset.x, pos.y),

            // Override y coordinate using pre-calculation offset of size including margins
            Mode::TopToBottom => vec2(pos.x, parent_pos.y + self.offset.y),

            // No overrides
            Mode::Align => pos,
        };

        // Spacing
        if let Mode::LeftToRight = p_mode {
            pos.x += p_spacing * p_idx as f32;
        } else if let Mode::TopToBottom = p_mode {
            pos.y += p_spacing * p_idx as f32;
        }

        // Margins
        if p_len > 0 {
            if p_mode == Mode::LeftToRight {
                pos.x += self.margins.left;
                pos.y += self.margins.top - self.margins.bottom;
            } else if p_mode == Mode::TopToBottom {
                pos.x += self.margins.left - self.margins.right;
                pos.y += self.margins.top;
            }
        } else {
            pos.x += self.margins.left - self.margins.right;
            pos.y += self.margins.top - self.margins.bottom;
        }

        pos
    }

    // Clone the layout and not just the layout reference
    fn copy(&self) -> SharedLayout {
        Rc::new(RefCell::new(self.clone()))
    }

    // Get sub-layout's index in this layout
    fn index(&self, id: &str) -> Option<usize> {
        self.subs.iter().position(|x| x.borrow().id == id)
    }
}

/// Layout describes a region of space and provides mechanisms for calculating where and how a
/// widget should draw itself inside that region of space. Layout region space allocated to widgets
/// is then tracked.
#[derive(Clone, Debug, PartialEq)]
pub struct Layout(SharedLayout);

// Constructors and builders
impl Layout {
    /// Create the default layout
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self(LayoutInner::new(id))
    }

    /// Create a horizontal layout
    /// * lays out sub-layouts using the left to right packing mode
    pub fn horz<T: AsRef<str>>(id: T) -> Self {
        let layout = Self::new(id);
        {
            let inner = &mut *layout.0.borrow_mut();
            inner.dirty = true;
            inner.mode = Mode::LeftToRight;
        }
        layout
    }

    /// Create a vertical layout
    /// * lays out sub-layouts using the top to bottom packing mode
    pub fn vert<T: AsRef<str>>(id: T) -> Self {
        let layout = Self::new(id);
        {
            let inner = &mut *layout.0.borrow_mut();
            inner.dirty = true;
            inner.mode = Mode::TopToBottom;
        }
        layout
    }

    /// Clone the layout and not just the layout reference
    /// * clone's layout properties and layout sub-layouts except parent
    /// * parent layout will remain the same shared layout as the original
    /// * returned layout id will need to be changed to make it unique
    pub fn copy(&self) -> Self {
        let layout = Layout(self.0.borrow().copy());
        {
            let inner = &mut *layout.0.borrow_mut();
            inner.subs.clear();
            for x in self.0.borrow().subs.iter() {
                let sub_layout = x.borrow().copy();
                sub_layout.borrow_mut().parent = Some(layout.0.clone());
                inner.subs.push(sub_layout);
            }
        }
        layout
    }

    /// Set layout alignment
    /// * controls this widgets alignment in its parent layout
    pub fn with_align(self, align: Align) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.align = align;
        }
        self
    }

    /// Set layout expansion to true
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
    pub fn with_fill_width(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = true;
        }
        self
    }

    /// Fill the entire height of the layout
    pub fn with_fill_height(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_h = true;
        }
        self
    }

    /// Set the layout's identifier
    pub fn with_id<T: AsRef<str>>(self, id: T) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.id = id.as_ref().to_string();
        }
        self
    }

    /// Space reserved outside the boundaries of the layout
    pub fn with_margins(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.margins = RectOffset {
                left,
                right,
                top,
                bottom,
            };
        }
        self
    }

    /// Set layout packing mode
    /// * lays out sub-layouts using the given mode
    pub fn with_mode(self, mode: Mode) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.mode = mode;
        }
        self
    }

    /// Set layout expansion to false
    pub fn with_no_expand(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.expand = false;
        }
        self
    }

    /// Set fill property to false
    pub fn with_no_fill(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = false;
            inner.fill_h = false;
        }
        self
    }

    /// Set fill width to false
    pub fn with_no_fill_width(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = false;
        }
        self
    }

    /// Set fill height to false
    pub fn with_no_fill_height(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_h = false;
        }
        self
    }

    /// Add a parent layout for relative alignment
    /// * when align is set the LayoutMode won't take affect
    pub fn with_parent(self, parent: Layout) -> Self {
        parent.append(&self);
        self
    }

    /// Set the layout size to full screen
    /// * disables layout expansion
    pub fn with_size_full(self) -> Self {
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
    pub fn with_size_percent(self, width: f32, height: f32) -> Self {
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
    pub fn with_size_static(self, width: f32, height: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.expand = false;
            inner.size = vec2(width, height);
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
}

// Getters and setters
impl Layout {
    /// Get layout alignment
    pub fn align(&self) -> Align {
        self.0.borrow().align
    }

    /// Get layout expand property
    pub fn expand(&self) -> bool {
        self.0.borrow().expand
    }

    /// Get layout fill will be true when both fill_width and fill_height are set to true
    pub fn fill(&self) -> bool {
        self.0.borrow().fill_w && self.0.borrow().fill_h
    }

    /// Get layout fill height property
    pub fn fill_height(&self) -> bool {
        self.0.borrow().fill_h
    }

    /// Get layout fill width property
    pub fn fill_width(&self) -> bool {
        self.0.borrow().fill_w
    }

    /// Get layout id property
    pub fn id(&self) -> String {
        self.0.borrow().id.clone()
    }

    /// Get layout margins
    pub fn margins(&self) -> RectOffset {
        self.0.borrow().margins
    }

    /// Get layout mode
    pub fn mode(&self) -> Mode {
        self.0.borrow().mode
    }

    /// Get layout parent
    pub fn parent(&self) -> Option<Layout> {
        self.0.borrow().parent.as_ref().map(|x| Layout(x.clone()))
    }

    /// Get parent layout's position and size
    /// * position accounts for margins
    /// * returns (pos, size)
    pub fn parent_shape(&self) -> (Vec2, Vec2) {
        match self.parent() {
            Some(parent) => {
                let (parent_pos, parent_size) = parent.parent_shape();
                (self.0.borrow().pos(parent_pos, parent_size), parent.size())
            },

            // Default parent position and size
            _ => (Vec2::default(), screen()),
        }
    }

    /// Get layout's position
    // * assumes layout size and parent size are already updated
    /// * position accounts for margins
    /// * returns (pos, size)
    pub fn pos(&self) -> Vec2 {
        let (parent_pos, parent_size) = self.parent_shape();
        self.0.borrow().pos(parent_pos, parent_size)
    }

    /// Get layout's position and size
    // * assumes layout size and position and parent size and positon are already updated
    /// * position accounts for margins
    /// * returns (pos, size)
    pub fn shape(&self) -> (Vec2, Vec2) {
        let (parent_pos, parent_size) = self.parent_shape();
        let pos = self.0.borrow().pos(parent_pos, parent_size);
        (pos, self.size())
    }

    /// Get layout size not including margins
    pub fn size(&self) -> Vec2 {
        self.0.borrow().size
    }

    /// Get layout spacing
    pub fn spacing(&self) -> f32 {
        self.0.borrow().spacing
    }

    /// Get the number of sub-layouts
    pub fn sub_len(&self) -> usize {
        self.0.borrow().subs.len()
    }

    /// Get sub-layout by id
    pub fn sub(&self, id: &str) -> Option<Layout> {
        self.0.borrow().subs.iter().find(|x| x.borrow().id == id).map(|x| Layout(x.clone()))
    }

    /// Get sub-layout's index in this layout
    pub fn sub_idx(&self, id: &str) -> Option<usize> {
        self.0.borrow().subs.iter().position(|x| x.borrow().id == id)
    }

    /// Get sub-layout's position and size by id
    /// * position accounts for margins
    /// * size accounts for margins
    /// * returns (pos, size)
    pub fn sub_shape(&self, id: &str) -> Option<(Vec2, Vec2)> {
        self.sub(id).map(|x| x.shape())
    }

    /// Set the layout's size not including margins
    pub fn set_size(&self, size: Vec2) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.expand = false;
        inner.size = size;
    }

    /// Set sub-layout by id
    pub fn set_sub(&self, sub_layout: Layout) {
        let inner = &mut *self.0.borrow_mut();
        if let Some(i) = inner.index(&sub_layout.id()) {
            inner.subs[i] = sub_layout.0.clone();
        } else {
            inner.subs.push(sub_layout.0.clone());
        }
        inner.dirty = true;
    }

    /// Set the sub-layout's size by id
    /// * same as `set_layout_size_s` but takes Vec2 object instead of floats
    pub fn set_sub_size(&self, id: &str, size: Vec2) {
        self.sub(id).map(|x| x.set_size(size));
    }

    // Create a new layout inside this layout
    fn alloc<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let mut layout = Layout::new(id.as_ref().to_string()).with_parent(Layout(self.0.clone()));
        if let Some(size) = size {
            layout = layout.with_size_static(size.x, size.y).with_expand();
        } else {
            layout = layout.with_expand();
        }
        layout
    }

    /// Create a new sub-layout inside this layout
    /// * Adds the new sub-layout to the end of the sub-layout list
    pub fn alloc_append<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let layout = self.alloc(id.as_ref(), size);
        let inner = &mut *self.0.borrow_mut();
        inner.subs.push(layout.0.clone());
        inner.dirty = true;
        layout
    }

    /// Create a new sub-layout inside this layout
    /// * Adds the new sub-layout to the begining of the sub-layout list
    pub fn alloc_prepend<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let layout = self.alloc(id.as_ref(), size);
        let inner = &mut *self.0.borrow_mut();
        inner.subs.insert(0, layout.0.clone());
        inner.dirty = true;
        layout
    }

    /// Append the given sub-layout to this layout
    /// * Adds the new sub-layout to the end of the sub-layout list if it doesn't already exist
    pub fn append(&self, layout: &Layout) {
        if self.sub_idx(&layout.id()).is_none() {
            {
                // Set parent on layout
                let sub = &mut *layout.0.borrow_mut();
                sub.parent = Some(self.0.clone());
                sub.dirty = true;

                // Update parent
                let inner = &mut *self.0.borrow_mut();
                inner.subs.push(layout.0.clone());
                inner.dirty = true;
            }
            layout.update();
            self.update();
        }
    }

    /// Calculate and set the size and positional offset of the layout and sub-layouts
    /// * only performs calculation if needed
    /// * returns the size calculation including margins
    pub fn update(&self) -> Vec2 {
        let (expand, mode, mut size) = {
            let inner = &mut *self.0.borrow_mut();

            // Calculate total layout size based on static size and margins
            let inner_size = vec2(
                inner.size.x + inner.margins.left + inner.margins.right,
                inner.size.y + inner.margins.top + inner.margins.bottom,
            );

            // Return persisted value including margins if not dirty
            if !inner.dirty {
                return inner_size;
            }

            inner.dirty = false;
            (inner.expand, inner.mode, inner_size)
        };

        // Calculate total layout size based on sub-layouts size and margins
        if !self.0.borrow().subs.is_empty() {
            size = Vec2::default();
            let mut offset = Vec2::default();
            for x in self.0.borrow().subs.iter() {
                x.borrow_mut().offset = offset; // Set positional offsets along the way.
                let sub_size = Layout(x.clone()).update();

                match mode {
                    Mode::LeftToRight | Mode::Align => {
                        size.x += sub_size.x;
                        if size.y < sub_size.y {
                            size.y = sub_size.y;
                        }
                        offset.x = size.x;
                    },
                    Mode::TopToBottom => {
                        if size.x < sub_size.x {
                            size.x = sub_size.x;
                        }
                        size.y += sub_size.y;
                        offset.y = size.y;
                    },
                }
            }

            // Persist the calculated size if set to expand
            if expand && size != Vec2::default() {
                self.0.borrow_mut().size = size;
            }
        }

        // Handle fill directives
        let inner = &mut *self.0.borrow_mut();
        if let Some(parent) = &inner.parent {
            let parent = parent.borrow();

            if parent.fill_w {
                inner.size.x = parent.size.x;
                size.x = parent.size.x;
            }
            if parent.fill_h {
                inner.size.y = parent.size.y;
                size.y = parent.size.x;
            }
        }

        size
    }
}

/// Define different layout modes
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    /// Pack widgets horizontally with vertical alignment
    LeftToRight,

    /// Pack widgets vertically with horizontal alignment
    TopToBottom,

    /// Pack widgets based on alignment only
    Align,
}

impl Default for Mode {
    /// Create the default Mode directive a.k.a Align
    ///
    /// ### Examples
    /// ```
    /// use specter::prelude::*;
    ///
    /// assert_eq!(Mode::default(), Mode::Align);
    /// ```
    fn default() -> Self {
        Mode::Align
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    //     // Alignment against parent
    //     let mut pos = self.align.relative(self.size, parent_size, parent_pos);
    //     pos = match parent_mode {
    //         // Override x coordinate using pre-calculation offset of size including margins
    //         Mode::LeftToRight => vec2(parent_pos.x + self.offset.x, pos.y),

    //         // Override y coordinate using pre-calculation offset of size including margins
    //         Mode::TopToBottom => vec2(pos.x, parent_pos.y + self.offset.y),

    //         // No overrides
    //         Mode::Align => pos,
    //     };

    //     // Spacing
    //     if let Mode::LeftToRight = parent_mode {
    //         pos.x += parent_spacing * parent_idx as f32;
    //     } else if let Mode::TopToBottom = parent_mode {
    //         pos.y += parent_spacing * parent_idx as f32;
    //     }

    //     // Margins
    //     if parent_len > 0 {
    //         if parent_mode == Mode::LeftToRight {
    //             pos.x += self.margins.left;
    //             pos.y += self.margins.top - self.margins.bottom;
    //         } else if parent_mode == Mode::TopToBottom {
    //             pos.x += self.margins.left - self.margins.right;
    //             pos.y += self.margins.top;
    //         }
    //     } else {
    //         pos.x += self.margins.left - self.margins.right;
    //         pos.y += self.margins.top - self.margins.bottom;
    //     }

    #[test]
    fn append_multiple() {
        let parent = Layout::new("parent").with_size_static(60., 60.).with_mode(Mode::LeftToRight);
        let (parent_pos, parent_size) = parent.shape();
        assert_eq!((parent_pos, parent_size), (vec2(0., 0.), vec2(60., 60.)));

        let size = vec2(20., 20.);
        let layout1 = Layout::new("id1").with_size_static(size.x, size.y).with_parent(parent.clone());
        let layout2 = Layout::new("id2").with_size_static(size.x, size.y).with_parent(parent.clone());

        // Check that sub-layouts are being appended to parent properly
        assert_eq!(parent.sub_len(), 2);
    }

    #[test]
    fn pos_alignment_mode_align() {
        let (w, h) = (450., 800.);
        let size = vec2(20., 20.);
        let layout = Layout::new("id1").with_size_static(size.x, size.y);
        let (parent_pos, parent_size) = layout.parent_shape();
        assert_eq!(layout.shape(), (vec2(0., 0.), size));
        assert_eq!((parent_pos, parent_size), (vec2(0., 0.), vec2(w, h)));

        let layout = layout.with_align(Align::Center);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));

        let layout = layout.with_align(Align::CenterBottom);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));

        let layout = layout.with_align(Align::CenterTop);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));

        let layout = layout.with_align(Align::LeftBottom);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));

        let layout = layout.with_align(Align::LeftCenter);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));

        let layout = layout.with_align(Align::LeftTop);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));

        let layout = layout.with_align(Align::RightBottom);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));

        let layout = layout.with_align(Align::RightCenter);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));

        let layout = layout.with_align(Align::RightTop);
        assert_eq!(layout.pos(), layout.align().relative(size, parent_size, parent_pos));
    }

    #[test]
    fn builder_functions() {
        let layout = Layout::new("id1");
        assert_eq!(layout.id(), "id1");
        assert_eq!(layout.size(), vec2(0., 0.));
        assert_eq!(layout.fill(), false);
        assert_eq!(layout.fill_height(), false);
        assert_eq!(layout.fill_width(), false);
        assert_eq!(layout.expand(), true);
        assert_eq!(layout.align(), Align::LeftTop);
        assert_eq!(layout.mode(), Mode::Align);
        assert_eq!(layout.spacing(), 0.);
        assert_eq!(layout.margins(), RectOffset::default());
        assert_eq!(layout.parent(), None);

        // Now change the layout properties with builder functions
        let layout = layout
            .with_id("id2")
            .with_size_static(1., 2.)
            .with_fill()
            .with_no_expand()
            .with_align(Align::Center)
            .with_mode(Mode::LeftToRight)
            .with_spacing(10.)
            .with_margins(1., 0., 0., 0.)
            .with_parent(Layout::new("parent1"));
        assert_eq!(layout.id(), "id2");
        assert_eq!(layout.size(), vec2(1., 2.));
        assert_eq!(layout.fill(), true);
        assert_eq!(layout.fill_height(), true);
        assert_eq!(layout.fill_width(), true);
        assert_eq!(layout.expand(), false);
        assert_eq!(layout.align(), Align::Center);
        assert_eq!(layout.mode(), Mode::LeftToRight);
        assert_eq!(layout.spacing(), 10.);
        assert_eq!(layout.margins(), RectOffset::new(1., 0., 0., 0.));
        assert_eq!(layout.parent().is_some(), true);
        assert_eq!(layout.parent().map(|x| x.id()).unwrap(), "parent1");

        // Test no fill builder functions
        let layout = layout.with_no_fill_height();
        assert_eq!(layout.fill_height(), false);
        let layout = layout.with_no_fill_width();
        assert_eq!(layout.fill_width(), false);
        let layout = layout.with_fill().with_no_fill();
        assert_eq!(layout.fill(), false);
    }
}
