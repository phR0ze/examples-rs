# Specter
Immediate mode Ui toolkit

### Quick links
* [Layout management](#layout-management)

* [Widget properties](#widget-properties)
  * [Builders](#builders)
  * [Getters](#getters)
  * [Setters](#setters)

## Layout management
Layout management is based on layout objects that provide policy and configuration to guide the 
automatic arrangement of child widgets within a parent widget including positioning and sizing 
dynamically for the available space. They are higher level constructs that allow you to move beyond 
exact postional coordinates. 

Immediate mode Ui's typically make use of lambdas liberally to stitch Ui code together much like a 
scripting language. However such models don't require a more primitive layout management system due 
to the lack of sizing knowldget ahead of time. Specter instead provides mechanisms for layout 
decisions to be made separate from actual excution and drawing of the Ui.

## Widget properties
Every wiget has a set of properties associated with it used to control styling and behaviors. The 
typical interactions with these properties is done through `builder`, `getter` and `setter` 
functions.

**Naming:**  
Because Rust doesn't support function overloading functions with related purposes e.g. builder, 
getter, or setter functions used to interact with common properties typically have a prefix or suffix 
describing the action being performed on the property. Only one of the 3 typical function types can 
have the simple un-prefixed/un-suffixed name. In `Specter`'s case that typically is the builder 
function as they are the most commonly used functions.

**Example Signatures:**
```rust
// Builders
fn id<T: AsRef<str>>(self, id: T) -> Self
fn size(self, size: Vec2) -> Self

// Getters
fn get_id(&self) -> &str
fn get_size(&self) -> Vec2

// Setters
fn set_id<T: AsRef<str>>(&mut self, id: T)
fn set_size(&mut self, size: Vec2)
```

### Builders
Builder functions are those used during widget construction to set optional properties. Builder 
functions consume the object `self` and return a new modified copy of the object `Self`.

**Naming:**  
Because Rust doesn't support function overloading the functions typically prefixed with `with_`. In 
Specter's case they are used so often I decided to not have the prefix and instead prefix the 
getters. Additionally surfacing internal objects for mutation via a lambda is very useful and 
ergonomic.

**Examples:**
```rust
/// Set the widget's id
pub fn id<T: AsRef<str>>(self, id: T) -> Self {
  Self { id: id.as_ref().to_string(), ..self }
}

/// Set the widget's size
pub fn size(self, size: Vec2) -> Self {
  Self { size, ..self }
}

/// Set the widget's frame
pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
  Self {
    frame: f(self.frame),
    ..self
  }
}
```

### Getters
Getters are functions that return the widget's internal properties as references, or copies of the 
properties, or calculate values based on internal data and return a reference or copy of the data. 
Getters take an immutable reference to the widget and are idempotent. 

**Naming:**  
Because Rust doesn't support function overloading the functions are simply called the property name 
or describe the calculation being performed. Additionally surfacing internal objects via a copy.
is very useful and ergonomic if somewhat prone to performance overhead.

**Examples:**
```rust
/// Return a reference to the widget's id
pub fn get_id(&self) -> &str {
  &self.id
}

/// Return a copy of the widget's (position, size)
pub fn shape(&self) -> (Vec2, Vec2) {
  (self.pos, self.size)
}

/// Get the widget's frame properties
pub fn get_frame(&self) -> &mut Self {
  self.frame = f(self.frame);
  self
}
```

### Setters
Setters are functions that set a widget's properties or update internal widget state. Typically they 
take a mutable reference to the widget or if not leverage interior mutability. 

**Naming:**  
Because Rust doesn't support function overloading the functions are prefixed with `set_` or describe 
the calculation being performed in a unique way to keep the function names unique from getter 
functions. Surfacing internal objects for mutation via a lambda is very useful and ergonomic.

**Examples:**
```rust
/// Set the widget's id
pub fn set_id<T: AsRef<str>>(&mut self, id: T) {
  self.id = id.as_ref().to_string();
}

/// Set the widget's size
pub fn set_size(&mut self, size: Vec2) {
  self.size = size;
}

/// Set the widget's frame
pub fn set_frame(&mut self, f: impl FnOnce(Frame) -> Frame) {
  self.frame = f(self.frame);
}
```

## Backlog
* Frame properties

* Layout support for label
* Layout inline nesting
* Layout tracks child layouts and updates them with positioning when parent changes

## Completed
* Layout percentage [examples/layout_fill_width.rs](examples/layout_fill_width.rs)
* Layout fill height [examples/layout_fill_height.rs](examples/layout_fill_height.rs)
* Layout fill width [examples/layout_fill_width.rs](examples/layout_fill_width.rs)
* Layout horizontal [examples/layout_vertical.rs](examples/layout_horizontal.rs)
* Layout vertical [examples/layout_vertical.rs](examples/layout_vertical.rs)
* Layout margins affect alignment [examples/button_align.rs](examples/button_align.rs)
* FPS widget with dark theme [examples/fps_dark.rs](examples/fps_dark.rs)
* FPS widget with smoothing affect [examples/fps_light.rs](examples/fps_light.rs)
* Button builder for reusable button styling [examples/button_align.rs](examples/button_align.rs)
* Layout label align [examples/button_align.rs](examples/button_align.rs)
* Layout align [examples/button_align.rs](examples/button_align.rs)
* Layout nested [examples/layout_nested.rs](examples/layout_nested.rs)
* Layout margins [examples/layout_vertical.rs](examples/layout_vertical.rs)
* Layout spacing [examples/layout_vertical.rs](examples/layout_vertical.rs)
* Button support for icon [examples/button_icon.rs](exmamples/button_icon.rs)
* Button support for toggling [examples/button_icon.rs](exmamples/button_icon.rs)
* Button support for label size [examples/button_icon.rs](exmamples/button_icon.rs)
* Button support for label color [examples/button_icon.rs](exmamples/button_icon.rs)
* Button support for background color [examples/button_icon.rs](exmamples/button_icon.rs)
