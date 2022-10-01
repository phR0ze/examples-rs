use std::time::Instant;

use macroquad::{
    prelude::*,
    ui::{root_ui, widgets, Skin, Ui},
};

// Mobile device screens have the same or better pixel density as full monitors
// but are tiny, so its necessary to scale up the rendered results.
#[cfg(not(target_os = "android"))]
pub const SCALE_MULTIPLIER: f32 = 1.0;
#[cfg(target_os = "android")]
pub const SIZE_MULTIPLIER: f32 = 4.0;

/// Default font size
pub const DEFAULT_FONT_SIZE: f32 = 30.0;

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

pub struct Fps {
    dirty: bool,        // track if the update skin function needs run
    skin: Option<Skin>, // skin cache for frames per second
    frames: u64,        // count the frames until the next second
    start: Instant,     // time to start tracking from
    fps: u16,           // last calculated frames per second
    font_color: Color,  // font color to use
}

impl Fps {
    pub fn new() -> Fps {
        Fps { dirty: true, skin: None, frames: 0, start: Instant::now(), fps: 0, font_color: BLACK }
    }

    /// Set the font color to use
    pub fn with_font_color(self, color: Color) -> Self {
        Fps { dirty: true, font_color: color, ..self }
    }

    /// Return the fps right now
    pub fn now(&self) -> u16 {
        self.fps
    }

    // Update the skin
    fn update_skin(&mut self, ui: &mut Ui) {
        if !self.dirty {
            return;
        }
        let label_style = ui
            .style_builder()
            .font_size(DEFAULT_FONT_SIZE as u16)
            .text_color(self.font_color)
            .text_color_hovered(self.font_color)
            .text_color_clicked(self.font_color)
            .build();
        self.skin = Some(Skin { label_style, ..ui.default_skin() });
        self.dirty = false;
    }

    /// Draw the frames per second in the top left of the screen
    pub fn ui(&mut self, ui: &mut Ui) {
        self.update_skin(ui);

        // Calculate fps
        self.frames += 1;
        let us = self.start.elapsed().as_micros();
        if us == 0 {
            // nothing to do
            return;
        }
        self.fps = ((self.frames * 1000000) as u128 / us) as u16;

        // Reset fps when we have 10sec worth of data
        if self.start.elapsed().as_secs() > 10 {
            self.frames = 0;
            self.start = Instant::now();
        }

        ui.push_skin(self.skin.as_ref().unwrap());
        widgets::Label::new(format!("FPS: {}", self.fps)).position(vec2(10., 10.)).ui(ui);
        ui.pop_skin();
    }
}
