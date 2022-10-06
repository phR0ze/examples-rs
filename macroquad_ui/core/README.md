# core
Utility functions and helpers

## Layout Management
After rewriting sizing and positioning code multiple times I think it might make sense to see how
main stream UI tool kits have solved this.
* [Gtk layout management](https://zetcode.com/gui/gtk2/gtklayoutmanagement/)
  * GtkAlignment
  * GtkHBox - single row
  * GtkVBox - single column
  * GtkTable - arrange by rows and columns
* [WxWidgets Sizers](https://zetcode.com/gui/wxwidgets/layoutmanagement/)
  * WxBoxSizer
    * Vertical or Horizontal layout
    * `.add(<widget>)` to add 
    * border between widgets
    * left, right, top, bottom, all, expand
    * 
  * WxStaticBoxSizer
  * wxGridSizer
  * wxFlexGridSizer
  * wxGridBagSizer
* [Qt5 layout management](https://zetcode.com/gui/qt5/layoutmanagement/)
  * HBoxLayout
  * VBoxLayout
  * GridLayout
  * FormLayout
  * Enum AlignRight

## Backlog
* Support for label to have clicked and hovered color changes
* Relatively size the button based on the containing widget's size
  * [examples/relative.rs](exmamples/relative.rs)
* Icon button: Macroquad doesn't allow for anything other than a label out of the box 
  * [examples/icon.rs](exmamples/icon.rs)
* Sizing directives to: [examples/sized.rs](exmamples/sized.rs)
  * Size button 1/2 of relative container
  * Size button 3/4 of relative container
  * Size button full size of relative container

## Completed
* Pass through support for button label size
* Pass through support for button label color
* Pass through support for button background color
* Button toggling: Macroquad button doesn't track a second click differently
* Label positioning: Macroquad button labels are always centered
  * Right align button label [examples/button_label_position.rs](exmamples/button_label_position.rs)
  * Left align button label