use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
};

// Mobile device screens have the same or better pixel density as full monitors
// but are tiny, so its necessary to scale up the rendered results.
#[cfg(not(target_os = "android"))]
pub const SCALE_MULTIPLIER: f32 = 1.0;
#[cfg(target_os = "android")]
pub const SIZE_MULTIPLIER: f32 = 4.0;

/// Default font size
pub const DEFAULT_FONT_SIZE: f32 = 30.0;

/// Returns a f32 that will scale up for mobile devices
pub fn scale(value: f32) -> f32 {
    value * SCALE_MULTIPLIER
}

/// Returns a MQ vec2 that will scale up for mobile devices
pub fn scale_vec2(x: f32, y: f32) -> Vec2 {
    vec2(x * SCALE_MULTIPLIER, y * SCALE_MULTIPLIER)
}

/// Returns a MQ RectOffset that will scale up for mobile devices
pub fn scale_rect(left: f32, right: f32, top: f32, bottom: f32) -> RectOffset {
    RectOffset::new(
        left * SCALE_MULTIPLIER,
        right * SCALE_MULTIPLIER,
        top * SCALE_MULTIPLIER,
        bottom * SCALE_MULTIPLIER,
    )
}

/// Calculate text size based on exact rendered text size
/// * `skin` requires the `label_style` be overridden to get accurate values
pub fn text_size(skin: &Skin, text: Option<&str>) -> Vec2 {
    let str = text.unwrap_or("default");
    root_ui().push_skin(skin);
    let size = root_ui().calc_size(str);
    root_ui().pop_skin();
    size
}
