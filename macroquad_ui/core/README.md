# core
Utility functions and helpers

## Containers

### Group
Group is the fundamental container object from which all others are derived

## Backlog
* Layout with frame

## Completed
* Layout nested [examples/layout_nested.rs](examples/layout_nested.rs)
* Layout size [examples/layout_percentage.rs](examples/layout_percentage.rs)
* Layout fill [examples/layout_fill_width.rs](examples/layout_fill_width.rs)
* Layout margin [examples/layout_horizontal.rs](examples/layout_horizontal.rs)
* Layout spacing [examples/layout_horizontal.rs](examples/layout_horizontal.rs)
* Layout horizontal [examples/layout_horizontal.rs](examples/layout_horizontal.rs)
* Layout vertical [examples/layout_vertical.rs](examples/layout_vertical.rs)

## Old Backlog
* Support for label to have clicked and hovered color changes
* Relatively size the button based on the containing widget's size
  * [examples/relative.rs](exmamples/relative.rs)
* Icon button: Macroquad doesn't allow for anything other than a label out of the box 
  * [examples/icon.rs](exmamples/icon.rs)
* Sizing directives to: [examples/sized.rs](exmamples/sized.rs)
  * Size button 1/2 of relative container
  * Size button 3/4 of relative container
  * Size button full size of relative container

## Old Completed
* Pass through support for button label size
* Pass through support for button label color
* Pass through support for button background color
* Button toggling: Macroquad button doesn't track a second click differently
* Label positioning: Macroquad button labels are always centered
  * Right align button label [examples/button_label_position.rs](exmamples/button_label_position.rs)
  * Left align button label