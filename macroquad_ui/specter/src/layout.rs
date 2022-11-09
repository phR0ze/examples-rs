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
#[derive(Debug, PartialEq)]
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

/// Layout describes a region of space and provides mechanisms for calculating where and how a
/// widget should draw itself inside that region of space. Layout region space allocated to widgets
/// is then tracked.
#[derive(Debug, PartialEq)]
pub struct Layout(SharedLayout);

impl Clone for Layout {
    /// Clone this layout not just the reference counter
    /// * sub-layouts will be deep copies and NOT references of the originals
    /// * sub-layout parent references will be updated to point to new clones
    /// * all cloned layouts will be marked to perform `update_size_and_offset`
    /// * returned layout will not have its parent set
    /// * returned layout will need its id updated to be unique
    fn clone(&self) -> Self {
        let layout = Layout::new("");
        {
            // Clone all properies
            let inner = self.0.borrow();
            let other = &mut *layout.0.borrow_mut();
            other.dirty = true; // new layout will need re-calculated
            other.offset = inner.offset;
            other.id = inner.id.clone();
            other.size = inner.size;
            other.fill_w = inner.fill_w;
            other.fill_h = inner.fill_h;
            other.expand = inner.expand;
            other.align = inner.align;
            other.mode = inner.mode;
            other.spacing = inner.spacing;
            other.margins = inner.margins;

            // Don't set the parent reference as we need to make a followup
            // call to set the parent outside this function or it will get confusing
            //other.parent = inner.parent.clone();

            // Clone all sub-layouts recursively
            for x in inner.subs.iter() {
                let sub = Layout(x.clone()).clone();

                // Set parent on layout after the fact
                {
                    let sub = &mut *sub.0.borrow_mut();
                    sub.parent = Some(layout.0.clone());
                    sub.dirty = true;
                }

                // Append sub-layout to parent
                other.subs.push(sub.0);
            }
        }
        layout
    }
}

// Constructors and builders
impl Layout {
    /// Create the default layout
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self(Rc::new(RefCell::new(LayoutInner {
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
        })))
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
    pub fn with_parent(self, parent: &Layout) -> Self {
        parent.subs_append(&self);
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

// Getters
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

    /// Get layout size not including margins
    pub fn size(&self) -> Vec2 {
        self.0.borrow().size
    }

    /// Get layout spacing
    pub fn spacing(&self) -> f32 {
        self.0.borrow().spacing
    }
}

// Setters
impl Layout {
    /// Set layout alignment
    /// * `align` is the alignment to set
    pub fn set_align(&self, align: Align) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.align = align;
    }

    /// Set layout expand property
    pub fn set_expand(&self) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.expand = true;
    }

    /// Set layout fill which sets fill_width and fill_height
    pub fn set_fill(&self) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.fill_w = true;
        inner.fill_h = true;
    }

    /// Set layout fill height property
    pub fn set_fill_height(&self) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.fill_h = true;
    }

    /// Set layout fill width property
    pub fn set_fill_width(&self) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.fill_w = true;
    }

    /// Set the layout's id
    pub fn set_id<T: AsRef<str>>(&self, id: T) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.id = id.as_ref().to_string();
    }

    /// Set the layout's size not including margins
    pub fn set_size(&self, size: Vec2) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.expand = false;
        inner.size = size;
    }

    /// Set layout margins
    /// * `margins` is the margins to set
    pub fn set_margins(&self, margins: RectOffset) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.margins = margins;
    }

    /// Set layout mode
    pub fn set_mode(&self, mode: Mode) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.mode = mode;
    }

    /// Set layout parent to reference internally
    /// * this layout's parent property will be linked to the given parent
    /// * `parent` is the parent layout to reference
    pub fn set_parent(&self, parent: &Layout) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.parent = Some(parent.0.clone());
    }

    /// Set layout spacing
    pub fn set_spacing(&self, spacing: f32) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.spacing = spacing;
    }
}

// Utility functions
impl Layout {
    /// Get parent layout's position and size
    /// * position accounts for margins
    /// * returns (pos, size)
    pub fn parent_shape(&self) -> (Vec2, Vec2) {
        match self.parent() {
            Some(parent) => parent.shape(),

            // Default parent position and size
            _ => (Vec2::default(), screen()),
        }
    }

    /// Get layout's position
    /// * assumes layout size and parent size are already updated
    /// * position accounts for margins
    /// * returns (pos, size)
    pub fn pos(&self) -> Vec2 {
        let (p_pos, p_size) = self.parent_shape();
        let (p_mode, p_spacing, p_idx, p_len) = match self.parent() {
            Some(parent) => (
                parent.mode(),
                parent.spacing(),
                parent.sub_idx(&self.0.borrow().id).unwrap_or(0) as f32,
                parent.subs_len(),
            ),
            _ => (Mode::default(), 0., 0., 0),
        };

        // Alignment against parent
        let inner = self.0.borrow();
        let mut pos = inner.align.relative(inner.size, p_size, p_pos);
        pos = match p_mode {
            // Override x coordinate using pre-calculation offset of size including margins
            Mode::LeftToRight => vec2(p_pos.x + inner.offset.x, pos.y),

            // Override y coordinate using pre-calculation offset of size including margins
            Mode::TopToBottom => vec2(pos.x, p_pos.y + inner.offset.y),

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
                pos.x += inner.margins.left;
                pos.y += inner.margins.top - inner.margins.bottom;
            } else if p_mode == Mode::TopToBottom {
                pos.x += inner.margins.left - inner.margins.right;
                pos.y += inner.margins.top;
            }
        } else {
            pos.x += inner.margins.left - inner.margins.right;
            pos.y += inner.margins.top - inner.margins.bottom;
        }

        pos
    }

    /// Create a reference of the layout to work with
    /// * calls clone on the internal Rc to get a new reference
    /// * useful for storing a single object in multiple locations
    pub fn rc_ref(&self) -> Layout {
        Layout(self.0.clone())
    }

    /// Returns true if the `other` layout is a pointer to this layout
    pub fn rc_ref_eq(&self, other: &Layout) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    /// Get layout's position and size
    // * assumes layout size and position and parent size and positon are already updated
    /// * position accounts for margins
    /// * returns (pos, size)
    pub fn shape(&self) -> (Vec2, Vec2) {
        self.update_size_and_offset();
        (self.pos(), self.size())
    }

    /// Get sub-layout by id
    pub fn sub(&self, id: &str) -> Option<Layout> {
        self.0.borrow().subs.iter().find(|x| x.borrow().id == id).map(|x| Layout(x.clone()))
    }

    // Create a new layout inside this layout
    fn sub_alloc<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let layout = Layout::new(id);
        {
            // Set parent on layout
            let sub = &mut *layout.0.borrow_mut();
            sub.parent = Some(self.0.clone());
            sub.dirty = true;

            // Set optional size
            if let Some(size) = size {
                sub.size = size;
            }

            // Ensure expand is set
            sub.expand = true;
        }
        layout
    }

    /// Create a new sub-layout inside this layout
    /// * Adds the new sub-layout to the end of the sub-layout list
    pub fn sub_alloc_append<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let sub = self.sub_alloc(id.as_ref(), size);
        let inner = &mut *self.0.borrow_mut();
        inner.subs.push(sub.0.clone());
        inner.dirty = true;
        sub
    }

    /// Create a new sub-layout inside this layout
    /// * Adds the new sub-layout to the begining of the sub-layout list
    pub fn sub_alloc_prepend<T: AsRef<str>>(&self, id: T, size: Option<Vec2>) -> Layout {
        let layout = self.sub_alloc(id.as_ref(), size);
        let inner = &mut *self.0.borrow_mut();
        inner.subs.insert(0, layout.0.clone());
        inner.dirty = true;
        layout
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

    /// Set the sub-layout's size by id
    /// * `id` is the sub-layout's id to set the size for
    /// * `size` is the sub-layout's size to set
    pub fn sub_set_size(&self, id: &str, size: Vec2) {
        self.sub(id).map(|x| x.set_size(size));
    }

    /// Append the given sub-layout to this layout
    /// * Adds the sub-layout to the end of the sub-layout list if it doesn't already exist
    /// * Calls update if the sub-layout was appended
    /// * `layout` is the sub-layout to append
    pub fn subs_append(&self, layout: &Layout) {
        if self.sub_idx(&layout.id()).is_none() {
            {
                // Set parent on layout
                let sub = &mut *layout.0.borrow_mut();
                sub.parent = Some(self.0.clone());
                sub.dirty = true;

                // Append sub-layout to parent
                let inner = &mut *self.0.borrow_mut();
                inner.subs.push(layout.0.clone());
                inner.dirty = true;
            }
        }
    }

    /// Get sub-layout by index
    pub fn subs_idx(&self, i: usize) -> Option<Layout> {
        if self.subs_len() > i {
            Some(Layout(self.0.borrow().subs[i].clone()))
        } else {
            None
        }
    }

    /// Get the number of sub-layouts
    pub fn subs_len(&self) -> usize {
        self.0.borrow().subs.len()
    }

    /// Update the sub-layout with the given sub-layout
    /// * Calls update after the sub-layout is replaced or appended
    /// * `layout` is the sub-layout to replace or append
    pub fn subs_update(&self, layout: &Layout) {
        let inner = &mut *self.0.borrow_mut();
        if let Some(i) = inner.subs.iter().position(|x| x.borrow().id == inner.id) {
            inner.subs[i] = layout.0.clone();
        } else {
            inner.subs.push(layout.0.clone());
        }
        inner.dirty = true;
    }

    /// Calculate and set the size and positional offset of the layout and sub-layouts
    /// * only performs calculation if needed
    /// * returns the size calculation including margins
    pub fn update_size_and_offset(&self) -> Vec2 {
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
                let sub_size = Layout(x.clone()).update_size_and_offset();

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
    fn empty() -> Vec2 {
        vec2(0., 0.)
    }

    #[test]
    fn left_to_right_expansion_no_align() {
        let parent = Layout::new("parent").with_mode(Mode::LeftToRight);
        assert_eq!(parent.shape(), (empty(), empty()));

        // Check expanding parent size
        // widget = accumulation, height = largest
        let size = vec2(20., 20.);
        let layout1 = Layout::new("id1").with_size_static(size.x, size.y).with_parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(20., 20.)));
        let layout2 = Layout::new("id2").with_size_static(size.x, size.y).with_parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(40., 20.)));
        let layout3 = Layout::new("id3").with_size_static(size.x, 30.).with_parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(60., 30.)));

        // Check that sub-layouts are being appended to parent properly
        assert_eq!(parent.subs_len(), 3);
        assert_eq!(parent.subs_idx(0).unwrap().rc_ref_eq(&layout1), true);
        assert_eq!(parent.subs_idx(1).unwrap().rc_ref_eq(&layout2), true);
        assert_eq!(parent.subs_idx(2).unwrap().rc_ref_eq(&layout3), true);
        assert_eq!(parent.subs_idx(0).unwrap().parent().unwrap().rc_ref_eq(&parent), true);
        assert_eq!(parent.subs_idx(1).unwrap().parent().unwrap().rc_ref_eq(&parent), true);
        assert_eq!(parent.subs_idx(2).unwrap().parent().unwrap().rc_ref_eq(&parent), true);
        assert_eq!(parent.shape(), (empty(), vec2(60., 30.)));

        // Check sub-layout position
        assert_eq!(layout1.shape(), (vec2(0., 0.), size));
        assert_eq!(layout2.shape(), (vec2(20., 0.), size));
        assert_eq!(layout3.shape(), (vec2(40., 0.), vec2(20., 30.)));

        // Now set spacing and check
        parent.set_spacing(5.);

        // Check sub-layout position
        assert_eq!(layout1.shape(), (vec2(0., 0.), size));
        assert_eq!(layout2.shape(), (vec2(25., 0.), size));
        assert_eq!(layout3.shape(), (vec2(50., 0.), vec2(20., 30.)));
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
    fn clone() {
        let parent1 = Layout::new("parent1");
        let layout1 = Layout::new("layout1")
            .with_size_static(1., 2.)
            .with_fill()
            .with_no_expand()
            .with_align(Align::CenterTop)
            .with_mode(Mode::TopToBottom)
            .with_spacing(10.)
            .with_margins(1., 0., 0., 0.)
            .with_parent(&parent1);
        let sub1 = Layout::new("sub1").with_parent(&layout1);
        let sub2 = Layout::new("sub2").with_parent(&layout1);

        // Test layout1 original values
        assert_eq!(layout1.parent().unwrap().rc_ref_eq(&parent1), true);
        assert_eq!(layout1.subs_len(), 2);
        assert_eq!(layout1.subs_idx(0).unwrap().rc_ref_eq(&sub1), true);
        assert_eq!(layout1.subs_idx(0).unwrap().rc_ref_eq(&sub2), false);
        assert_eq!(layout1.subs_idx(1).unwrap().rc_ref_eq(&sub2), true);
        assert_eq!(layout1.subs_idx(1).unwrap().rc_ref_eq(&sub1), false);
        assert_eq!(layout1.subs_idx(0).unwrap().parent().unwrap().rc_ref_eq(&layout1), true);
        assert_eq!(layout1.subs_idx(1).unwrap().parent().unwrap().rc_ref_eq(&layout1), true);

        // Test layout2 clone values
        let layout2 = layout1.clone().with_id("layout2");
        assert_eq!(layout2.id(), "layout2");
        assert_eq!(layout2.size(), vec2(1., 2.));
        assert_eq!(layout2.fill(), true);
        assert_eq!(layout2.fill_height(), true);
        assert_eq!(layout2.fill_width(), true);
        assert_eq!(layout2.expand(), false);
        assert_eq!(layout2.align(), Align::CenterTop);
        assert_eq!(layout2.mode(), Mode::TopToBottom);
        assert_eq!(layout2.spacing(), 10.);
        assert_eq!(layout2.margins(), RectOffset::new(1., 0., 0., 0.));

        // Check that parent wasn't included
        assert_eq!(layout2.parent().is_none(), true);

        // Check subs we're actually cloned
        assert_eq!(layout2.subs_len(), 2);
        assert_eq!(layout2.subs_idx(0).unwrap().rc_ref_eq(&sub1), false);
        assert_eq!(layout2.subs_idx(0).unwrap().rc_ref_eq(&sub2), false);
        assert_eq!(layout2.subs_idx(1).unwrap().rc_ref_eq(&sub2), false);
        assert_eq!(layout2.subs_idx(1).unwrap().rc_ref_eq(&sub1), false);

        // Check subs have new parent
        assert_eq!(layout2.subs_idx(0).unwrap().parent().unwrap().rc_ref_eq(&layout2), true);
        assert_eq!(layout2.subs_idx(1).unwrap().parent().unwrap().rc_ref_eq(&layout2), true);
    }

    #[test]
    fn rc_ref_eq() {
        // Same pointer
        let parent1 = Layout::new("parent1");
        let layout1 = Layout::new("layout1").with_parent(&parent1);
        let layout2 = layout1.rc_ref();
        assert_eq!(layout1.rc_ref_eq(&layout2), true);
        assert_eq!(layout1.parent().unwrap().rc_ref_eq(&parent1), true);
        assert_eq!(layout1.parent().unwrap().rc_ref_eq(&layout1), false);

        // Different pointer and no parent
        let layout2 = layout1.clone();
        assert_eq!(layout1.rc_ref_eq(&layout2), false);
        assert_eq!(layout1.parent().unwrap().rc_ref_eq(&parent1), true);
        assert_eq!(layout2.parent().is_none(), true);
    }

    #[test]
    fn builder_functions() {
        // Getter functions
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

        // Setter functions
        layout.set_id("id2");
        assert_eq!(layout.id(), "id2");
        layout.set_size(vec2(0., 2.));
        assert_eq!(layout.size(), vec2(0., 2.));
        layout.set_fill_height();
        assert_eq!(layout.fill_height(), true);
        layout.set_fill_width();
        assert_eq!(layout.fill_width(), true);
        layout.set_fill();
        assert_eq!(layout.fill(), true);
        layout.set_align(Align::Center);
        assert_eq!(layout.align(), Align::Center);
        layout.set_expand();
        assert_eq!(layout.expand(), true);
        layout.set_mode(Mode::LeftToRight);
        assert_eq!(layout.mode(), Mode::LeftToRight);
        layout.set_spacing(5.);
        assert_eq!(layout.spacing(), 5.);
        layout.set_margins(RectOffset::new(2., 0., 0., 0.));
        assert_eq!(layout.margins(), RectOffset::new(2., 0., 0., 0.));
        layout.set_parent(&Layout::new("parent1"));
        assert_eq!(layout.parent().is_some(), true);
        assert_eq!(layout.parent().map(|x| x.id()).unwrap(), "parent1");

        // Builder functions
        let layout = layout
            .with_id("id3")
            .with_size_static(1., 2.)
            .with_fill()
            .with_no_expand()
            .with_align(Align::CenterTop)
            .with_mode(Mode::TopToBottom)
            .with_spacing(10.)
            .with_margins(1., 0., 0., 0.)
            .with_parent(&Layout::new("parent2"));
        assert_eq!(layout.id(), "id3");
        assert_eq!(layout.size(), vec2(1., 2.));
        assert_eq!(layout.fill(), true);
        assert_eq!(layout.fill_height(), true);
        assert_eq!(layout.fill_width(), true);
        assert_eq!(layout.expand(), false);
        assert_eq!(layout.align(), Align::CenterTop);
        assert_eq!(layout.mode(), Mode::TopToBottom);
        assert_eq!(layout.spacing(), 10.);
        assert_eq!(layout.margins(), RectOffset::new(1., 0., 0., 0.));
        assert_eq!(layout.parent().is_some(), true);
        assert_eq!(layout.parent().map(|x| x.id()).unwrap(), "parent2");

        // Test no fill builder functions
        let layout = layout.with_no_fill_height();
        assert_eq!(layout.fill_height(), false);
        let layout = layout.with_no_fill_width();
        assert_eq!(layout.fill_width(), false);
        let layout = layout.with_fill().with_no_fill();
        assert_eq!(layout.fill(), false);
    }
}
