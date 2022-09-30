# menu
Experimenting with writing a flexible menu widget

## Mobile notes
* Macroquad buttons don't allow for label positioning. they are always centered
* Separating out root_ui uses as a runtime borrow issue will occur if we don't
  allow each usage to complete out before trying to do another operation that
  depends on root_ui
* Everything needs to be zoomed by 4x to be visible on Android
* Using a window size of 400x800 seems to visually be similar to the Android emulator
* Google releases their new icons for Android apps on a periodic basis
  * https://fonts.google.com/icons
  * https://github.com/google/material-design-icons/

## Backlog
* Align menu entries on left

## Completed
* Create new Button to allow for label positioning inside button
* Menu entry fills width
* Menu entry background color is settable
* Menu background color is settable
* Menu: size is ThreeQuarter screen and full height
* Menu: position top left without margin
* Options: size is half screen minus margin by static height
* Options: position top right with optional margin
* Run menu on Android
* Padding inside entry
* Padding inside group
* Support for spacing between menu entries
* Menu to reposition relative to application sizing