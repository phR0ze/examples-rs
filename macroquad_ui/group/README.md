# Group
Minimal widget grouper without titles, close buttons or other niceties provided by other
widgets like Window. Simply povides relative positioning inside a box.

### Quick links
* [Scrolling](#scrolling)
  * [Disable auto scrolling](#disable-auto-scrolling)
* [Margins](#margins)
* [Backlog](#backlog)
* [Completed](#completed)

## Supported
* automatically scrolls if content is too large for stated size
* position - shifts the group to a new location moving all widgets inside the box as well
* draggable
* hoverable
* highlight

## Layout
* Layout property seems to have no affect if Horizontal or Vertical?

## Background
Macroquad's `background` style properties have no effect on the group. I've addressed this with my 
Group wrapper.

**Examples**:
* [examples/background.rs](examples/background.rs)

## Margins
Macroquad style `margin` and `background_margin` have no effect when a static `size()` is set for a 
widget. Since the group constructor requires a static size this means that margin values will never 
be used. I've addressed this with my group wrapper.

**Examples**:
* [examples/margins.rs](examples/margins.rs)

## Scrolling
A `Group` will automatically scroll if its content exceeds the group's original size. Note that the 
scrollbar will take up the original group's size and will not flow over that original size. see 

**Examples**:
* [examples/scrolling.rs](examples/scrolling.rs)

### Disable auto scrolling
Group's auto scrolling functionality can be disabled by setting the following styles changes

```rust
let scroll_width = 0.0;
let scroll_multiplier = 0.0;
let scrollbar_style = ui.style_builder().color(BLANK).color_hovered(BLANK).color_clicked(BLANK).build();
let scrollbar_handle_style = ui.style_builder().color(BLANK).color_hovered(BLANK).color_clicked(BLANK).build();
let skin = Skin {
    scrollbar_style,
    scrollbar_handle_style,
    scroll_width,
    scroll_multiplier,
    ..ui.default_skin()
};
```

## Backlog

## Completed
* Auto sized based on width and height directives [examples/relative.rs](examples/relative.rs)
* Support padding inside group space [examples/margins.rs](examples/margins.rs)
* Support disabling scrolling [examples/scrolling.rs](examples/scrolling.rs)
* Support backgrounds [examples/background.rs](examples/background.rs)
