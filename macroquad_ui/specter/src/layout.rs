//! Layout describes a region of space in which widgets should be drawn and provides mechanisms for
//! calculating and tracking where and how they should be drawn.
//!
//! ## Defaults
//! * Align layout
//! * Expansion enabled
//!
//! ## Terminology
//! * `child layout` is the inner layout when dealing with nested layouts
//! * `parent layout` is the outer layout when dealing with nested layouts
//! * `origin` is contextual to a layout and refers to where its top left corner starts
//! * `mode` is the packing directive defining different layout packing interpretations
//!
//! ## Pack mode
//! Pack modes provide different interpretations of how child layouts should be packed into the
//! parent layout's defined region of content space. The default `Mode::LeftToRight` will add child
//! layouts horizontally from left to right while `Mode::TopToBottom` will add child layouts
//! vertically from top to bottom. `Mode::Align` enables the use of the Align directive without
//! which the Align directive will be only partially adhered to on child layouts.
//!
//! ## Align directive
//! The align directive is used to remove a layout from the standard linear packing mode to instead
//! follow a calculated positioning relative to its parent layout. Margins and padding will still
//! affect the position even when an align directive is used but behave differently then the
//! standard linear packing modes. While the standard linear modes process out to in, first parent
//! margins then parent padding then child margins; Align will take padding into account during the
//! relative calculation but margins are processed post alignment. For example if a layout, sized
//! 20x20, is to be positioned in the center of its parent, sized 100x100, its origin would be
//! (40,40). If we set a child margin (not parent margin) of 10.0 on the layout alignment would be
//! calculated as (40,40) then the margin would be included to make the final origin as (50,50) with
//! a post shift in the origin. Padding however is calculated as a reduction in parent content space
//! which means in a similar situation if we instead had a parent padding of 5.0 alignment would be
//! calculated still as (40, 40) because of the even reduction in content space of the parent.
//!
//! Mixed modes make for partial adhereance to packing rules. When a parent layout is set to Align
//! and a child layout is set to a linear mode both modes will be respected intuitively with the
//! child layout being computed in a linear fashion inside the parent and the parent being
//! positioned according to the alignment rules. However if a parent layout is set to be a linear
//! layout and a child layout is set to `Mode::Align` behavior is a mixure. Specifically linear
//! rules will be followed for the direction of packing. This means that with `Mode::LeftToRight`
//! horizontal linear rules will be followed for the child despite the child's align directive yet
//! vertical rules such as top, center or bottom including margins will be followed for the child's
//! align directive. Likewise for `Mode::TopToBottom` vertical linear rules will be for the child
//! layout despite the child's align directive yet horiztonal align rules such as left, center and
//! right including margins will be followed for the child's align directive.
//!
//! ## Expand directive
//! Layout expansion is the default mode. In this mode the layout will expand its size to account
//! for the size of all content. This is very useful for cases where you don't know the size of the
//! layout in advance and need to build that knowledge based on the layouts it is composed of taking
//! into account margins, spacing and/or alignment preferences for one or more widgts. For example a
//! Button is composed of an Icon and Label each of which can be measured and included into the
//! total layout expansion. Setting the expand directive will disable the fill directives.
//!
//! ## Fill directive
//! The fill properties `fill_w`, `fill_h` and `fill` direct sub-layouts to fill the layout's width
//! (w), height (h) or both directions. This provides the ability to create a Panel to be used as a
//! menu with a fixed size and then have buttons of unknown size fill the width of the menu with
//! margins taken into acocunt. Setting the fill directives will cause the expand directive to be
//! ignored. Setting fill directives requires the layout to have a static size configured.
//!
//! ## Spacing
//! When packing child layouts in a parent layout consecutively, spacing can be applied to provide a
//! consistent space between child layouts. This directive is only used for non alignment modes like
//! LeftToRight or TopToBottom. It is not considered when using the Align mode.
//!
//! ## Margins
//! Margins are defined as additional space outside the widget's content area. Margins will affect
//! how the widget is drawn inside a parent layout in any mode. Margins are included in the size of
//! the widget when considering how much space a widget will require inside a parent widget added to
//! the total size a parent layout will expand to. Additionally the widget's position inside the
//! parent layout will be affected. A left margin of 5.0 pts will positionally offset the widget to
//! the right by 5.0 pts.
//!
//! ## Padding
//! Padding is defined as additional space inside the widget's content area. Padding will affect how
//! widgets are drawn inside its content area in any mode. It will push widget's in from the edges
//! of the content area. Padding is included in the widget's size.
//!
//! ## Overflow control
//! Overflow occurs when a child layout's position or size spans its parent layout's border. When
//! overflow control is enabled and overflow is detected corrective actions will be taken to adjust
//! the child layout to fit within the parent layout. First the child layout will be repositioned as
//! much as possible. If this still doesn't move the child layout within the parent layout's borders
//! the child layout will then be resized to fit within the parent layout's content space.
use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};

/// SharedLayout defines a sharable interior mutable Layout object
type SharedLayout = Rc<RefCell<LayoutInner>>;

// Internal implemenation detail for sharing ownership of layouts
#[derive(Debug, PartialEq)]
struct LayoutInner {
    // Internal only
    dirty: bool,  // track if calculations are needed
    offset: Vec2, // calculated positional offset

    // Exposed through Layout functions
    id: String,                   // layout identifier
    pos: Vec2,                    // caching for calculated position
    size: Vec2,                   // size of the layout region excluding margins
    fill_w: bool,                 // fill width of layout
    fill_h: bool,                 // fill height of layout
    expand: bool,                 // layout expands to track all content allocated
    align: Align,                 // alignment in the parent layout
    mode: Mode,                   // layout mode directive
    spacing: f32,                 // space to include between widgets
    margins: RectOffset,          // space outside the frame edge
    padding: RectOffset,          // space inside the frame edge
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
            other.pos = inner.pos;
            other.size = inner.size;
            other.fill_w = inner.fill_w;
            other.fill_h = inner.fill_h;
            other.expand = inner.expand;
            other.align = inner.align;
            other.mode = inner.mode;
            other.spacing = inner.spacing;
            other.margins = inner.margins;
            other.padding = inner.padding;

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
            pos: Vec2::default(),
            size: Vec2::default(),
            offset: Vec2::default(),
            fill_w: false,
            fill_h: false,
            expand: true, // enable expansion by default
            mode: Mode::default(),
            align: Align::default(),
            spacing: 0.,
            margins: RectOffset::default(),
            padding: RectOffset::default(),
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
    /// * controls this widget's alignment in its parent layout
    pub fn align(self, align: Align) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.align = align;
        }
        self
    }

    /// Set layout expansion to true
    /// * When enabled disables fill properties
    pub fn expand(self) -> Self {
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
    pub fn fill(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = true;
            inner.fill_h = true;
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
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_h = true;
        }
        self
    }

    /// Set the layout's identifier
    pub fn id<T: AsRef<str>>(self, id: T) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.id = id.as_ref().to_string();
        }
        self
    }

    /// Space reserved outside the boundaries of the layout
    pub fn margins(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
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

    /// Space reserved outside the boundaries of the layout
    pub fn margins_all(self, margin: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.margins = RectOffset::new(margin, margin, margin, margin);
        }
        self
    }

    /// Set layout packing mode
    /// * lays out sub-layouts using the given mode
    pub fn mode(self, mode: Mode) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.mode = mode;
        }
        self
    }

    /// Set layout expansion to false
    pub fn no_expand(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.expand = false;
        }
        self
    }

    /// Set fill property to false
    pub fn no_fill(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = false;
            inner.fill_h = false;
        }
        self
    }

    /// Set fill width to false
    pub fn no_fill_w(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_w = false;
        }
        self
    }

    /// Set fill height to false
    pub fn no_fill_h(self) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.fill_h = false;
        }
        self
    }

    /// Space reserved inside the boundaries of the layout
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.padding = RectOffset {
                left,
                right,
                top,
                bottom,
            };
        }
        self
    }

    /// Space reserved inside the boundaries of the layout
    pub fn padding_all(self, pad: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.padding = RectOffset::new(pad, pad, pad, pad);
        }
        self
    }

    /// Add a parent layout for relative alignment
    /// * when align is set the LayoutMode won't take affect
    pub fn parent(self, widget: impl Widget) -> Self {
        let parent = widget.layout_g();
        parent.subs_append(&self);
        self
    }

    /// Set the layout size to full screen
    /// * disables layout expansion
    pub fn size_f(self) -> Self {
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
    pub fn size_p(self, width: f32, height: f32) -> Self {
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
    pub fn size_s(self, width: f32, height: f32) -> Self {
        {
            let inner = &mut *self.0.borrow_mut();
            inner.dirty = true;
            inner.expand = false;
            inner.size = vec2(width, height);
        }
        self
    }

    /// Space to allocate between widgets
    pub fn spacing(self, spacing: f32) -> Self {
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
    pub fn get_align(&self) -> Align {
        self.0.borrow().align
    }

    /// Get layout expand property
    pub fn get_expand(&self) -> bool {
        self.0.borrow().expand
    }

    /// Get layout fill will be true when both fill_width and fill_height are set to true
    pub fn get_fill(&self) -> bool {
        self.0.borrow().fill_w && self.0.borrow().fill_h
    }

    /// Get layout fill height property
    pub fn get_fill_height(&self) -> bool {
        self.0.borrow().fill_h
    }

    /// Get layout fill width property
    pub fn get_fill_width(&self) -> bool {
        self.0.borrow().fill_w
    }

    /// Get layout id property
    pub fn get_id(&self) -> String {
        self.0.borrow().id.clone()
    }

    /// Get layout margins
    pub fn get_margins(&self) -> RectOffset {
        self.0.borrow().margins
    }

    /// Get layout mode
    pub fn get_mode(&self) -> Mode {
        self.0.borrow().mode
    }

    /// Get layout padding
    pub fn get_padding(&self) -> RectOffset {
        self.0.borrow().padding
    }

    /// Get layout parent
    pub fn get_parent(&self) -> Option<Layout> {
        self.0.borrow().parent.as_ref().map(|x| Layout(x.clone()))
    }

    /// Get layout spacing
    pub fn get_spacing(&self) -> f32 {
        self.0.borrow().spacing
    }

    // Private functions
    // ---------------------------------------------------------------------------------------------

    /// Get cached position value
    fn pos(&self) -> Vec2 {
        self.0.borrow().pos
    }

    /// Get layout size includin padding but not including margins
    fn size(&self) -> Vec2 {
        self.0.borrow().size
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
    /// * disables the fill directives
    pub fn set_expand(&self) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.expand = true;
        inner.fill_w = false;
        inner.fill_h = false;
    }

    /// Set layout fill which sets fill_width and fill_height
    pub fn set_fill(&self) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.fill_w = true;
        inner.fill_h = true;
    }

    /// Set layout fill height property
    pub fn set_fill_h(&self) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.fill_h = true;
    }

    /// Set layout fill width property
    pub fn set_fill_w(&self) {
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
    /// * disables the expand directive
    pub fn set_size(&self, width: f32, height: f32) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.expand = false;
        inner.size = vec2(width, height);
    }

    /// Set layout margins
    pub fn set_margins(&self, left: f32, right: f32, top: f32, bottom: f32) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.margins = RectOffset::new(left, right, top, bottom);
    }

    /// Set layout margins
    /// * `margin` is the margins to set for all values
    pub fn set_margins_all(&self, margin: f32) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.margins = RectOffset::new(margin, margin, margin, margin);
    }

    /// Set layout mode
    pub fn set_mode(&self, mode: Mode) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.mode = mode;
    }

    /// Set layout padding
    pub fn set_padding(&self, left: f32, right: f32, top: f32, bottom: f32) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.padding = RectOffset::new(left, right, top, bottom);
    }

    /// Set layout padding values all to the one given
    /// * `pad` is the padding value to use for all
    pub fn set_padding_all(&self, pad: f32) {
        let inner = &mut *self.0.borrow_mut();
        inner.dirty = true;
        inner.padding = RectOffset::new(pad, pad, pad, pad);
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
    /// Returns true if the `other` layout is a pointer to this layout
    pub fn eq(&self, other: &Layout) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
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
    /// * returns (pos, size)
    pub fn sub_shape(&self, id: &str) -> Option<(Vec2, Vec2)> {
        self.sub(id).map(|x| x.shape())
    }

    /// Set the sub-layout's size by id
    /// * `id` is the sub-layout's id to set the size for
    /// * `size` is the sub-layout's size to set
    pub fn sub_set_size(&self, id: &str, width: f32, height: f32) {
        self.sub(id).map(|x| x.set_size(width, height));
    }

    /// Get an iterator over the sub-layouts
    pub fn iter(&self) -> impl Iterator<Item = Layout> {
        self.0.borrow().subs.iter().map(|x| Layout(x.clone())).collect::<Vec<Layout>>().into_iter()
    }

    /// Append the given sub-layout to this layout
    /// * Adds the sub-layout to the end of the sub-layout list if it doesn't already exist
    /// * Calls update if the sub-layout was appended
    /// * `layout` is the sub-layout to append
    pub fn subs_append(&self, layout: &Layout) {
        if self.sub_idx(&layout.get_id()).is_none() {
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

    /// Get layout's position and size
    /// * re-calculates the entire layout stack associated with this layout
    /// * returns (pos, size)
    pub fn shape(&self) -> (Vec2, Vec2) {
        let mut layout = self.layout_g();
        while let Some(parent) = layout.get_parent() {
            layout = parent.layout_g();
        }
        // if self.dirty {
        layout.update_size();
        layout.update_pos();
        // self.dirty = false;
        //}
        (self.pos(), self.size())
    }

    /// Recursive calculation of final position from outer most to inner
    /// * requires update_size to be already run
    /// * returns pos
    fn update_pos(&self) -> Vec2 {
        // Extract parent values
        let (p_pos, p_size, p_mode, p_pad) = match self.get_parent() {
            Some(parent) => (parent.pos(), parent.size(), parent.get_mode(), parent.get_padding()),
            _ => (Vec2::default(), screen(), Mode::default(), RectOffset::default()),
        };

        // Extract layout values
        let (mut size, align, offset, margins, mode) = {
            let inner = self.0.borrow();
            (inner.size, inner.align, inner.offset, inner.margins, inner.mode)
        };

        // Calculate position for alignment
        let mut pos = {
            // First reduce parent size by parent padding amount
            let padded = vec2(p_size.x - p_pad.left - p_pad.right, p_size.y - p_pad.top - p_pad.bottom);

            // Calculate position relative to parent
            let mut pos = align.relative(size, padded);

            // Absolute alignment shouldn't be modified
            if !align.is_absolute() {
                // Offset by parent layout's position which already includes its margins
                pos += p_pos;

                // Offset by parent layout's padding
                pos += vec2(p_pad.left, p_pad.top);

                // Offset by layout's margins
                pos += vec2(margins.left, margins.top);
            }
            pos
        };

        // Override controlled linear direction with pre-calculated value but use
        // centering alignment for non-controled linear direction.
        // Positional offset already handles margins and padding appropriately
        if p_mode != Mode::Align {
            if p_mode == Mode::LeftToRight {
                pos.x = p_pos.x + offset.x;
            } else {
                pos.y = p_pos.y + offset.y;
            }
        }

        // Overflow control
        let overflow = vec2(
            (pos.x + size.x + margins.right + p_pad.right) - (p_pos.x + p_size.x),
            (pos.y + size.y + margins.bottom + p_pad.bottom) - (p_pos.y + p_size.y),
        );
        if overflow.x > 0. {
            // Adjust position first
            pos.x -= overflow.x;

            // Now adjust size
            if pos.x < p_pos.x + p_pad.left + margins.left {
                let resize = (pos.x - (p_pos.x + p_pad.left + margins.left)).abs();
                size.x -= resize;
                pos.x += resize;
            }
        }
        if overflow.y >= 0. {
            // Adjust position first
            pos.y -= overflow.y;

            // Now adjust size
            if pos.y < p_pos.y + p_pad.top + margins.top {
                let resize = (pos.y - (p_pos.y + p_pad.top + margins.top)).abs();
                size.y -= resize;
                pos.y += resize;
            }
        }

        // Persist the calculated values
        {
            let mut inner = &mut *self.0.borrow_mut();
            inner.pos = pos;
            inner.size = size;
        }

        // Recurse on child layouts
        for x in self.0.borrow().subs.iter() {
            Layout(x.clone()).update_pos();
        }

        pos
    }

    /// Recursive calculationg of sizing and position from inner most layouts to outer
    /// * returns the size calculation including margins
    fn update_size(&self) -> Vec2 {
        let (expand, mode, mut size, margins, padding) = {
            let inner = &mut *self.0.borrow_mut();

            // Include margins in the total size for use in expansion cases
            let inner_size = vec2(
                inner.size.x + inner.margins.left + inner.margins.right,
                inner.size.y + inner.margins.top + inner.margins.bottom,
            );

            (inner.expand, inner.mode, inner_size, inner.margins, inner.padding)
        };

        // Calculate total layout size
        if !self.0.borrow().subs.is_empty() {
            // Add parent padding open
            size = vec2(padding.left, padding.top);
            let mut offset = size;

            let len = self.subs_len();
            for (i, x) in self.0.borrow().subs.iter().enumerate() {
                // Get sub-layout size and margins
                let (sub_size, sub_margins) = {
                    let sub = Layout(x.clone());
                    (sub.update_size(), sub.get_margins())
                };

                // Add sub-layout opening margins
                if mode == Mode::LeftToRight {
                    offset.x += sub_margins.left;
                    if offset.y < padding.top + sub_margins.top {
                        offset.y = padding.top + sub_margins.top;
                    }
                } else if mode == Mode::TopToBottom {
                    if offset.x < padding.left + sub_margins.left {
                        offset.x = padding.left + sub_margins.left;
                    }
                    offset.y += sub_margins.top;
                }
                x.borrow_mut().offset = offset;

                // Add sub-layout size, margins and spacing
                if mode == Mode::LeftToRight {
                    size.x += sub_size.x + self.add_spacing(i, len);
                    offset.x += sub_size.x - sub_margins.left + self.add_spacing(i, len);
                    if size.y < padding.top + sub_size.y {
                        size.y = padding.top + sub_size.y;
                    }
                } else if mode == Mode::TopToBottom {
                    if size.x < padding.left + sub_size.x {
                        size.x = padding.left + sub_size.x;
                    }
                    size.y += sub_size.y + self.add_spacing(i, len);
                    offset.y += sub_size.y - sub_margins.top + self.add_spacing(i, len);
                }
            }

            // Add closing padding
            size += vec2(padding.right, padding.bottom);

            // Persist the calculated size if set to expand
            if expand && size != Vec2::default() {
                self.0.borrow_mut().size = size;
            }

            // Add margins - don't persist
            size.x += margins.left + margins.right;
            size.y += margins.top + margins.bottom;
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

        // Returned values always includes margins
        size
    }

    // Returns spacing if between elements and spacing is non-zero
    fn add_spacing(&self, i: usize, len: usize) -> f32 {
        let mut value = 0.;
        let spacing = self.0.borrow().spacing;

        // Spacing is non-zero
        if spacing > 0. {
            // There is at least 2 elements else there is no between elements
            if len > 1 {
                // Index is not the end
                if i < len - 1 {
                    value = spacing;
                }
            }
        }

        value
    }
}

impl Widget for Layout {
    /// Get the widget's layout as a cloned reference
    fn layout_g(&self) -> Layout {
        Layout(self.0.clone())
    }
}

impl Widget for &Layout {
    /// Get the widget's layout as a cloned reference
    fn layout_g(&self) -> Layout {
        Layout(self.0.clone())
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
    fn layout_fill_height() {
        let p1 = Layout::horz("0").size_f().margins_all(10.).fill_h().spacing(10.);
        let layout1 = Layout::new("0").size_s(100., 100.).parent(&p1);
        let layout2 = Layout::new("1").size_s(100., 100.).parent(&p1);
        let layout3 = Layout::new("2").size_s(150., 100.).parent(&p1);

        assert_eq!(layout1.shape(), (vec2(10., 10.), vec2(100., 780.)));
        assert_eq!(layout2.shape(), (vec2(120., 10.), vec2(100., 780.)));
        assert_eq!(layout3.shape(), (vec2(230., 10.), vec2(150., 780.)));
    }

    #[test]
    fn layout_fill_width() {
        let p1 = Layout::vert("0").size_p(0.75, 1.).margins_all(10.).fill_w().spacing(10.);
        let layout1 = Layout::new("0").size_s(100., 100.).parent(&p1);
        let layout2 = Layout::new("1").size_s(100., 100.).parent(&p1);
        let layout3 = Layout::new("2").size_s(100., 150.).parent(&p1);

        assert_eq!(layout1.shape(), (vec2(10., 10.), vec2(337.5, 100.)));
        assert_eq!(layout2.shape(), (vec2(10., 120.), vec2(337.5, 100.)));
        assert_eq!(layout3.shape(), (vec2(10., 230.), vec2(337.5, 150.)));
    }

    #[test]
    fn layout_alignment() {
        let p1 = Layout::new("0").size_f();
        assert_eq!(p1.shape(), (vec2(0., 0.), vec2(450., 800.)));

        let size = vec2(100., 100.);
        let builder = Layout::new("").size_s(size.x, size.y);

        // All alignment permutations
        let layout1 = builder.clone().id("1").align(Align::Center).parent(&p1);
        let layout2 = builder.clone().id("2").align(Align::CenterBottom).parent(&p1);
        let layout3 = builder.clone().id("3").align(Align::CenterTop).parent(&p1);
        let layout4 = builder.clone().id("4").align(Align::LeftBottom).parent(&p1);
        let layout5 = builder.clone().id("5").align(Align::LeftCenter).parent(&p1);
        let layout6 = builder.clone().id("6").align(Align::LeftTop).parent(&p1);
        let layout7 = builder.clone().id("7").align(Align::RightBottom).parent(&p1);
        let layout8 = builder.clone().id("8").align(Align::RightCenter).parent(&p1);
        let layout9 = builder.clone().id("9").align(Align::RightTop).parent(&p1);
        let layout10 = builder.clone().id("10").align(Align::Absolute(175., 150.)).parent(&p1);

        let shapes = vec![
            vec2(175., 350.),
            vec2(175., 700.),
            vec2(175., 0.),
            vec2(0., 700.),
            vec2(0., 350.),
            vec2(0., 0.),
            vec2(350., 700.),
            vec2(350., 350.),
            vec2(350., 0.),
            vec2(175., 150.),
        ];

        for i in 0..p1.subs_len() {
            assert_eq!(p1.subs_idx(i).unwrap().shape(), (shapes[i], size));
        }
        assert_eq!(p1.shape(), (vec2(0., 0.), vec2(450., 800.)));

        // Spacing should have no affect when using alignment
        p1.set_spacing(10.);
        for i in 0..p1.subs_len() {
            assert_eq!(p1.subs_idx(i).unwrap().shape(), (shapes[i], size));
        }
        assert_eq!(p1.shape(), (vec2(0., 0.), vec2(450., 800.)));

        // Padding should offset alignment for sub-layouts inside parent
        p1.set_padding_all(30.);
        let shapes = vec![
            vec2(175., 350.),
            vec2(175., 670.),
            vec2(175., 30.),
            vec2(30., 670.),
            vec2(30., 350.),
            vec2(30., 30.),
            vec2(320., 670.),
            vec2(320., 350.),
            vec2(320., 30.),
            vec2(175., 150.),
        ];

        for i in 0..p1.subs_len() {
            assert_eq!(p1.subs_idx(i).unwrap().shape(), (shapes[i], size));
        }
        assert_eq!(p1.shape(), (vec2(0., 0.), vec2(450., 800.)));

        // Margins should offset alignment for sub-layouts inside parent
        p1.set_margins_all(10.);
        assert_eq!(layout1.shape(), (vec2(175., 350.), size));
        assert_eq!(layout2.shape(), (vec2(175., 660.), size));
        assert_eq!(layout3.shape(), (vec2(175., 40.), size));
        assert_eq!(layout4.shape(), (vec2(40., 660.), size));
        assert_eq!(layout5.shape(), (vec2(40., 350.), size));
        assert_eq!(layout6.shape(), (vec2(40., 40.), size));
        assert_eq!(layout7.shape(), (vec2(310., 660.), size));
        assert_eq!(layout8.shape(), (vec2(310., 350.), size));
        assert_eq!(layout9.shape(), (vec2(310., 40.), size));
        assert_eq!(layout10.shape(), (vec2(175., 150.), size));
        assert_eq!(p1.shape(), (vec2(10., 10.), vec2(430., 780.)));
    }

    #[test]
    fn layout_combination() {
        let p1 = Layout::new("p1").mode(Mode::TopToBottom).size_f().spacing(10.).padding_all(30.).margins_all(10.);

        // Row 1
        let r1 = Layout::new("r1")
            .mode(Mode::LeftToRight)
            .align(Align::Center)
            .spacing(10.)
            .padding_all(20.)
            .parent(&p1);
        let r1c1 = Layout::new("c1").size_s(100., 100.).parent(&r1);
        let r1c2 = Layout::new("c2").size_s(100., 100.).parent(&r1);
        let r1c3 = Layout::new("c3").size_s(100., 100.).parent(&r1);

        // Row 2
        let r2 = Layout::new("r2")
            .mode(Mode::LeftToRight)
            .align(Align::Center)
            .spacing(10.)
            .padding_all(20.)
            .parent(&p1);
        let r2c1 = Layout::new("c1").size_s(100., 100.).parent(&r2);
        let r2c2 = Layout::new("c2").size_s(100., 100.).parent(&r2);
        let r2c3 = Layout::new("c3").size_s(100., 100.).parent(&r2);

        assert_eq!(p1.shape(), (vec2(10., 10.), vec2(430., 780.)));
        assert_eq!(r1.shape().0, vec2(45., 40.));
        assert_eq!(r1c1.shape().0, vec2(65., 60.));
        assert_eq!(r1c2.shape().0, vec2(175., 60.));
        assert_eq!(r1c3.shape().0, vec2(285., 60.));
        assert_eq!(r2.shape().0, vec2(45., 190.));
        assert_eq!(r2c1.shape().0, vec2(65., 210.));
        assert_eq!(r2c2.shape().0, vec2(175., 210.));
        assert_eq!(r2c3.shape().0, vec2(285., 210.));
    }

    #[test]
    fn layout_vertical() {
        let parent = Layout::vert("0");

        // A linear layout in expansion mode will be empty until a child layout is added
        assert_eq!(parent.shape(), (empty(), empty()));

        // Check expanding parent size
        let size = vec2(100., 100.);
        let layout1 = Layout::new("1").size_s(size.x, size.y).parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(100., 100.)));
        let layout2 = Layout::new("2").size_s(size.x, size.y).parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(100., 200.)));
        let layout3 = Layout::new("3").size_s(150., size.y).parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(150., 300.)));

        // Check that sub-layouts are being appended to parent properly
        assert_eq!(parent.subs_len(), 3);
        assert_eq!(parent.subs_idx(0).unwrap().eq(&layout1), true);
        assert_eq!(parent.subs_idx(1).unwrap().eq(&layout2), true);
        assert_eq!(parent.subs_idx(2).unwrap().eq(&layout3), true);
        assert_eq!(parent.subs_idx(0).unwrap().get_parent().unwrap().eq(&parent), true);
        assert_eq!(parent.subs_idx(1).unwrap().get_parent().unwrap().eq(&parent), true);
        assert_eq!(parent.subs_idx(2).unwrap().get_parent().unwrap().eq(&parent), true);

        // Check shape
        assert_eq!(layout1.shape(), (vec2(0., 0.), size));
        assert_eq!(layout2.shape(), (vec2(0., 100.), size));
        assert_eq!(layout3.shape(), (vec2(0., 200.), vec2(150., 100.)));
        assert_eq!(parent.shape(), (empty(), vec2(150., 300.)));

        // Set parent spacing and check
        parent.set_spacing(10.);
        assert_eq!(layout1.shape(), (vec2(0., 0.), size));
        assert_eq!(layout2.shape(), (vec2(0., 110.), size));
        assert_eq!(layout3.shape(), (vec2(0., 220.), vec2(150., 100.)));
        assert_eq!(parent.shape(), (empty(), vec2(150., 320.)));

        // Set parent padding and check
        // Sub-layout positional offset will be affected but not parent
        parent.set_padding_all(20.);
        assert_eq!(layout1.shape(), (vec2(20., 20.), size));
        assert_eq!(layout2.shape(), (vec2(20., 130.), size));
        assert_eq!(layout3.shape(), (vec2(20., 240.), vec2(150., 100.)));
        assert_eq!(parent.shape(), (empty(), vec2(190., 360.)));

        // Set parent margins and check
        // Sub-layout positional offset will be affected and parent's
        parent.set_margins_all(10.);
        assert_eq!(layout1.shape(), (vec2(30., 30.), size));
        assert_eq!(layout2.shape(), (vec2(30., 140.), size));
        assert_eq!(layout3.shape(), (vec2(30., 250.), vec2(150., 100.)));
        assert_eq!(parent.shape(), (vec2(10., 10.), vec2(190., 360.)));

        // Set sub-layout margins and check
        // Because calling shape will calculate the parent as is we need to ensure all changes are done
        // before it is called.
        layout1.set_margins_all(10.);
        layout2.set_margins_all(10.);
        layout3.set_margins_all(10.);
        assert_eq!(layout1.shape(), (vec2(40., 40.), size));
        assert_eq!(layout2.shape(), (vec2(40., 170.), size));
        assert_eq!(layout3.shape(), (vec2(40., 300.), vec2(150., 100.)));
        assert_eq!(parent.shape(), (vec2(10., 10.), vec2(210., 420.)));

        // Set parent static size bigger than needed
        // Nothing should change other than the parent's size
        parent.set_size(230., 450.);
        assert_eq!(layout1.shape(), (vec2(40., 40.), size));
        assert_eq!(layout2.shape(), (vec2(40., 170.), size));
        assert_eq!(layout3.shape(), (vec2(40., 300.), vec2(150., 100.)));
        assert_eq!(parent.shape(), (vec2(10., 10.), vec2(230., 450.)));

        // Set parent static size too small for content
        // Overflow control should kick in and push content in at the end
        parent.set_size(210., 410.);
        assert_eq!(layout1.shape(), (vec2(40., 40.), size));
        assert_eq!(layout2.shape(), (vec2(40., 170.), size));
        assert_eq!(layout3.shape(), (vec2(40., 290.), vec2(150., 100.)));
        assert_eq!(parent.shape(), (vec2(10., 10.), vec2(210., 410.)));
    }

    #[test]
    fn layout_horizontal() {
        let parent = Layout::horz("0");

        // A linear layout in expansion mode will be empty until a child layout is added
        assert_eq!(parent.shape(), (empty(), empty()));

        // Check expanding parent size
        let size = vec2(100., 100.);
        let layout1 = Layout::new("1").size_s(size.x, size.y).parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(100., 100.)));
        let layout2 = Layout::new("2").size_s(size.x, size.y).parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(200., 100.)));
        let layout3 = Layout::new("3").size_s(size.x, 150.).parent(&parent);
        assert_eq!(parent.shape(), (empty(), vec2(300., 150.)));

        // Check that sub-layouts are being appended to parent properly
        assert_eq!(parent.subs_len(), 3);
        assert_eq!(parent.subs_idx(0).unwrap().eq(&layout1), true);
        assert_eq!(parent.subs_idx(1).unwrap().eq(&layout2), true);
        assert_eq!(parent.subs_idx(2).unwrap().eq(&layout3), true);
        assert_eq!(parent.subs_idx(0).unwrap().get_parent().unwrap().eq(&parent), true);
        assert_eq!(parent.subs_idx(1).unwrap().get_parent().unwrap().eq(&parent), true);
        assert_eq!(parent.subs_idx(2).unwrap().get_parent().unwrap().eq(&parent), true);

        // Check shape
        assert_eq!(layout1.shape(), (vec2(0., 0.), size));
        assert_eq!(layout2.shape(), (vec2(100., 0.), size));
        assert_eq!(layout3.shape(), (vec2(200., 0.), vec2(100., 150.)));
        assert_eq!(parent.shape(), (empty(), vec2(300., 150.)));

        // Set parent spacing and check
        parent.set_spacing(10.);
        assert_eq!(layout1.shape(), (vec2(0., 0.), size));
        assert_eq!(layout2.shape(), (vec2(110., 0.), size));
        assert_eq!(layout3.shape(), (vec2(220., 0.), vec2(100., 150.)));
        assert_eq!(parent.shape(), (empty(), vec2(320., 150.)));

        // Set parent padding and check
        // Sub-layout positional offset will be affected but not parent
        parent.set_padding_all(20.);
        assert_eq!(layout1.shape(), (vec2(20., 20.), size));
        assert_eq!(layout2.shape(), (vec2(130., 20.), size));
        assert_eq!(layout3.shape(), (vec2(240., 20.), vec2(100., 150.)));
        assert_eq!(parent.shape(), (empty(), vec2(360., 190.)));

        // Set parent margins and check
        // Sub-layout positional offset will be affected and parent's
        parent.set_margins_all(10.);
        assert_eq!(layout1.shape(), (vec2(30., 30.), size));
        assert_eq!(layout2.shape(), (vec2(140., 30.), size));
        assert_eq!(layout3.shape(), (vec2(250., 30.), vec2(100., 150.)));
        assert_eq!(parent.shape(), (vec2(10., 10.), vec2(360., 190.)));

        // Set sub-layout margins and check
        // Because calling shape will calculate the parent as is we need to ensure all changes are done
        // before it is called.
        layout1.set_margins_all(10.);
        layout2.set_margins_all(10.);
        layout3.set_margins_all(10.);
        assert_eq!(layout1.shape(), (vec2(40., 40.), size));
        assert_eq!(layout2.shape(), (vec2(170., 40.), size));
        assert_eq!(layout3.shape(), (vec2(300., 40.), vec2(100., 150.)));
        assert_eq!(parent.shape(), (vec2(10., 10.), vec2(420., 210.)));

        // Set parent static size bigger than needed
        // Nothing should change other than the parent's size
        parent.set_size(430., 230.);
        assert_eq!(layout1.shape(), (vec2(40., 40.), size));
        assert_eq!(layout2.shape(), (vec2(170., 40.), size));
        assert_eq!(layout3.shape(), (vec2(300., 40.), vec2(100., 150.)));
        assert_eq!(parent.shape(), (vec2(10., 10.), vec2(430., 230.)));

        // Set parent static size too small for content
        // Overflow control should kick in and push content in at the end
        parent.set_size(410., 210.);
        assert_eq!(layout1.shape(), (vec2(40., 40.), size));
        assert_eq!(layout2.shape(), (vec2(170., 40.), size));
        assert_eq!(layout3.shape(), (vec2(290., 40.), vec2(100., 150.)));
        assert_eq!(parent.shape(), (vec2(10., 10.), vec2(410., 210.)));
    }

    #[test]
    fn clone() {
        let parent1 = Layout::new("parent1");
        let layout1 = Layout::new("layout1")
            .size_s(1., 2.)
            .fill()
            .no_expand()
            .align(Align::CenterTop)
            .mode(Mode::TopToBottom)
            .spacing(10.)
            .margins(1., 0., 0., 0.)
            .parent(&parent1);
        let sub1 = Layout::new("sub1").parent(&layout1);
        let sub2 = Layout::new("sub2").parent(&layout1);

        // Test layout1 original values
        assert_eq!(layout1.get_parent().unwrap().eq(&parent1), true);
        assert_eq!(layout1.subs_len(), 2);
        assert_eq!(layout1.subs_idx(0).unwrap().eq(&sub1), true);
        assert_eq!(layout1.subs_idx(0).unwrap().eq(&sub2), false);
        assert_eq!(layout1.subs_idx(1).unwrap().eq(&sub2), true);
        assert_eq!(layout1.subs_idx(1).unwrap().eq(&sub1), false);
        assert_eq!(layout1.subs_idx(0).unwrap().get_parent().unwrap().eq(&layout1), true);
        assert_eq!(layout1.subs_idx(1).unwrap().get_parent().unwrap().eq(&layout1), true);

        // Test layout2 clone values
        let layout2 = layout1.clone().id("layout2");
        assert_eq!(layout2.get_id(), "layout2");
        assert_eq!(layout2.size(), vec2(1., 2.));
        assert_eq!(layout2.get_fill(), true);
        assert_eq!(layout2.get_fill_height(), true);
        assert_eq!(layout2.get_fill_width(), true);
        assert_eq!(layout2.get_expand(), false);
        assert_eq!(layout2.get_align(), Align::CenterTop);
        assert_eq!(layout2.get_mode(), Mode::TopToBottom);
        assert_eq!(layout2.get_spacing(), 10.);
        assert_eq!(layout2.get_margins(), RectOffset::new(1., 0., 0., 0.));

        // Check that parent wasn't included
        assert_eq!(layout2.get_parent().is_none(), true);

        // Check subs we're actually cloned
        assert_eq!(layout2.subs_len(), 2);
        assert_eq!(layout2.subs_idx(0).unwrap().eq(&sub1), false);
        assert_eq!(layout2.subs_idx(0).unwrap().eq(&sub2), false);
        assert_eq!(layout2.subs_idx(1).unwrap().eq(&sub2), false);
        assert_eq!(layout2.subs_idx(1).unwrap().eq(&sub1), false);

        // Check subs have new parent
        assert_eq!(layout2.subs_idx(0).unwrap().get_parent().unwrap().eq(&layout2), true);
        assert_eq!(layout2.subs_idx(1).unwrap().get_parent().unwrap().eq(&layout2), true);
    }

    #[test]
    fn rc_ref_eq() {
        // Same pointer
        let parent1 = Layout::new("parent1");
        let layout1 = Layout::new("layout1").parent(&parent1);
        let layout2 = layout1.layout_g();
        assert_eq!(layout1.eq(&layout2), true);
        assert_eq!(layout1.get_parent().unwrap().eq(&parent1), true);
        assert_eq!(layout1.get_parent().unwrap().eq(&layout1), false);

        // Different pointer and no parent
        let layout2 = layout1.clone();
        assert_eq!(layout1.eq(&layout2), false);
        assert_eq!(layout1.get_parent().unwrap().eq(&parent1), true);
        assert_eq!(layout2.get_parent().is_none(), true);
    }

    #[test]
    fn builder_functions() {
        // Getter functions
        let layout = Layout::new("id1");
        assert_eq!(layout.get_id(), "id1");
        assert_eq!(layout.size(), vec2(0., 0.));
        assert_eq!(layout.get_fill(), false);
        assert_eq!(layout.get_fill_height(), false);
        assert_eq!(layout.get_fill_width(), false);
        assert_eq!(layout.get_expand(), true);
        assert_eq!(layout.get_align(), Align::LeftTop);
        assert_eq!(layout.get_mode(), Mode::Align);
        assert_eq!(layout.get_spacing(), 0.);
        assert_eq!(layout.get_margins(), RectOffset::default());
        assert_eq!(layout.get_parent(), None);

        // Setter functions
        layout.set_id("id2");
        assert_eq!(layout.get_id(), "id2");
        layout.set_size(0., 2.);
        assert_eq!(layout.size(), vec2(0., 2.));
        layout.set_fill_h();
        assert_eq!(layout.get_fill_height(), true);
        layout.set_fill_w();
        assert_eq!(layout.get_fill_width(), true);
        layout.set_fill();
        assert_eq!(layout.get_fill(), true);
        layout.set_align(Align::Center);
        assert_eq!(layout.get_align(), Align::Center);
        layout.set_expand();
        assert_eq!(layout.get_expand(), true);
        layout.set_mode(Mode::LeftToRight);
        assert_eq!(layout.get_mode(), Mode::LeftToRight);
        layout.set_spacing(5.);
        assert_eq!(layout.get_spacing(), 5.);
        layout.set_margins(2., 0., 0., 0.);
        assert_eq!(layout.get_margins(), RectOffset::new(2., 0., 0., 0.));
        layout.set_parent(&Layout::new("parent1"));
        assert_eq!(layout.get_parent().is_some(), true);
        assert_eq!(layout.get_parent().map(|x| x.get_id()).unwrap(), "parent1");

        // Builder functions
        let layout = layout
            .id("id3")
            .size_s(1., 2.)
            .fill()
            .no_expand()
            .align(Align::CenterTop)
            .mode(Mode::TopToBottom)
            .spacing(10.)
            .margins(1., 0., 0., 0.)
            .parent(&Layout::new("parent2"));
        assert_eq!(layout.get_id(), "id3");
        assert_eq!(layout.size(), vec2(1., 2.));
        assert_eq!(layout.get_fill(), true);
        assert_eq!(layout.get_fill_height(), true);
        assert_eq!(layout.get_fill_width(), true);
        assert_eq!(layout.get_expand(), false);
        assert_eq!(layout.get_align(), Align::CenterTop);
        assert_eq!(layout.get_mode(), Mode::TopToBottom);
        assert_eq!(layout.get_spacing(), 10.);
        assert_eq!(layout.get_margins(), RectOffset::new(1., 0., 0., 0.));
        assert_eq!(layout.get_parent().is_some(), true);
        assert_eq!(layout.get_parent().map(|x| x.get_id()).unwrap(), "parent2");

        // Test no fill builder functions
        let layout = layout.no_fill_h();
        assert_eq!(layout.get_fill_height(), false);
        let layout = layout.no_fill_w();
        assert_eq!(layout.get_fill_width(), false);
        let layout = layout.fill().no_fill();
        assert_eq!(layout.get_fill(), false);
    }
}
