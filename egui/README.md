# egui

## Pros
* Drawing primitives i.e. `painter` is easily exposed and leveragable
  * Button implementation is simply done via painter
* Awesome feature for scaling all UI components with `pixels_per_point`
* Reactive mode only repaints when there is user interaction or animations

## Cons
* Doesn't seem to be a way to turn off button hovering visual affects
* No way to control just button size manually without also controlling positioning manually
* Button is drawn at origin then redrawn at desired location with ***visible artifact and delay between***
* Same ***visible artifact and delay of positioning*** occurs when just drawing a simple rectangle
