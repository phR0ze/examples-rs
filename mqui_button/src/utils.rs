use macroquad::{
    prelude::*,
    ui::{root_ui, Skin, Ui},
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

/// Instantiate a RectOffset
pub fn rect(left: f32, right: f32, top: f32, bottom: f32) -> Option<RectOffset> {
    Some(RectOffset::new(left, right, top, bottom))
}

/// Returns a MQ RectOffset that will scale up for mobile devices
pub fn scale_rectp(rect: RectOffset) -> RectOffset {
    RectOffset::new(
        rect.left * SCALE_MULTIPLIER,
        rect.right * SCALE_MULTIPLIER,
        rect.top * SCALE_MULTIPLIER,
        rect.bottom * SCALE_MULTIPLIER,
    )
}

/// Calculate text size based on exact rendered text size
/// * `skin` requires the `label_style` be overridden to get accurate values
pub fn text_size(ui: &mut Ui, skin: &Skin, text: Option<&str>) -> Vec2 {
    let str = text.unwrap_or("default");
    ui.push_skin(skin);
    let size = ui.calc_size(str);
    ui.pop_skin();
    size
}
