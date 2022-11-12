# Specter
Immediate mode Ui toolkit

## Widget properties
Every wiget has a set of properties associated with it used to control styling and behaviors. The 
typical interactions with these properties is done through `getter`, `setter` and `builder` 
functions.

**Example Signatures:**
```rust
fn id(&self) -> &str
fn size(&self) -> Vec2
fn set_id<T: AsRef<str>>(&mut self, id: T)
fn set_size(&mut self, size: Vec2)
fn with_id<T: AsRef<str>>(self, id: T) -> Self
fn with_size(self, size: Vec2) -> Self
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
pub fn id(&self) -> &str {
  &self.id
}

/// Return a copy of the widget's (position, size)
pub fn shape(&self) -> (Vec2, Vec2) {
  (self.pos, self.size)
}

/// Get the widget's frame properties
pub fn frame(&self) -> &mut Self {
  self.frame = f(self.frame);
  self
}

/// Get the widget's frame properties
pub fn frame(&self) -> Frame {
  self.frame
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

### Builders
Builder functions are those used during widget construction to set optional properties. Builder 
functions consume the object `self` and return a new modified copy of the object `Self`.

**Naming:**  
Because Rust doesn't support function overloading the functions are prefixed with `with_`. However in 
the case where you have a pure builder object the `with_` prefix can typically be dropped unless 
getters are being used on the builder object as well. Additionally surfacing internal objects for 
mutation via a lambda is very useful and ergonomic.

**Examples:**
```rust
/// Set the widget's id
pub fn with_id<T: AsRef<str>>(self, id: T) -> Self {
  Self { id: id.as_ref().to_string(), ..self }
}

/// Set the widget's size
pub fn with_size(self, size: Vec2) -> Self {
  Self { size, ..self }
}

/// Set the widget's frame
pub fn with_frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
  Self {
    frame: f(self.frame),
    ..self
  }
}
```

## Backlog
* Widgets taking a lambda of child layouts needs to have a predefined size currently as the prarent 
widget needs to be drawn before the child widgets are added to the parent for sizing.
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
