use macroquad::{
    prelude::*,
    ui::{Skin, Ui},
};

// Mobile device screens have the same or better pixel density as full monitors
// but are tiny, so its necessary to scale up the rendered results.
#[cfg(not(target_os = "android"))]
pub const SCALE_MULTIPLIER: f32 = 1.0;
#[cfg(target_os = "android")]
pub const SIZE_MULTIPLIER: f32 = 4.0;

/// Default font size
pub const DEFAULT_FONT_SIZE: f32 = 30.0;

/// Return the current screen size
pub fn screen() -> Vec2 {
    vec2(screen_width(), screen_height())
}

/// Instantiate a RectOffset
pub fn rect(left: f32, right: f32, top: f32, bottom: f32) -> Option<RectOffset> {
    Some(RectOffset::new(left, right, top, bottom))
}

/// Calculate text size based on exact rendered text size
/// * `skin` requires the `label_style` be overridden to get accurate values
pub fn text_size(ui: &mut Ui, skin: &Skin, text: Option<&str>) -> Vec2 {
    let str = text.unwrap_or("Settings"); // upper case S and lower case g give good sizing
    ui.push_skin(skin);
    let size = ui.calc_size(str);
    ui.pop_skin();
    size
}

/// Returns a f32 scaled up for mobile devices
pub fn scale(value: f32) -> f32 {
    value * SCALE_MULTIPLIER
}

/// Returns a vec2 scaled up for mobile devices
pub fn scale_vec2(x: f32, y: f32) -> Vec2 {
    vec2(x * SCALE_MULTIPLIER, y * SCALE_MULTIPLIER)
}

/// Returns a RectOffset scaled up for mobile devices
pub fn scale_rect(left: f32, right: f32, top: f32, bottom: f32) -> RectOffset {
    RectOffset::new(
        left * SCALE_MULTIPLIER,
        right * SCALE_MULTIPLIER,
        top * SCALE_MULTIPLIER,
        bottom * SCALE_MULTIPLIER,
    )
}

/// Returns a RectOffset scaled up for mobile devices
pub fn scale_rect_p(rect: RectOffset) -> RectOffset {
    RectOffset::new(
        rect.left * SCALE_MULTIPLIER,
        rect.right * SCALE_MULTIPLIER,
        rect.top * SCALE_MULTIPLIER,
        rect.bottom * SCALE_MULTIPLIER,
    )
}
