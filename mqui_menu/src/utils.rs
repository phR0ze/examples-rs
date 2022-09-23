use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};

// Mobile device screens have the same or better pixel density as full monitors
// but are tiny, so its necessary to scale up the rendered results.
#[cfg(not(target_os = "android"))]
pub const SCALE_MULTIPLIER: f32 = 1.0;
#[cfg(target_os = "android")]
pub const SIZE_MULTIPLIER: f32 = 4.0;

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
