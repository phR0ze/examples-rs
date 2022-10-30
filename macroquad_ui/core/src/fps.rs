//! Fps provides a simple frames per second widget to be displayed for debug purposes.
//! * Fps value is averaged over the last 10 seconds for a smoother appearance

use crate::align::*;
use crate::layout::PackMode;
use crate::utils::*;
use macroquad::{
    prelude::*,
    ui::{widgets, Skin, Ui},
};
use std::time::Instant;

pub struct Fps {
    fps: u16,           // last calculated frames per second
    dirty: bool,        // track if the update skin function needs run
    skin: Option<Skin>, // skin cache for frames per second
    frames: u64,        // count the frames until the next second
    start: Instant,     // time to start tracking from
    font_color: Color,  // font color to use
    position: Align,    // positional directive for location
}

impl Fps {
    /// Create a new Fps light instance
    pub fn new() -> Fps {
        Fps {
            dirty: true,
            fps: 0,
            skin: None,
            frames: 0,
            start: Instant::now(),
            font_color: BLACK,
            position: Align::LeftTop,
        }
    }

    /// Create a new Fps dark instance
    pub fn dark() -> Fps {
        Fps::new().with_font_color(WHITE)
    }

    /// Set the position
    pub fn with_position(self, position: Align) -> Self {
        Fps { position, ..self }
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

    /// Draw the frames per second as directed
    pub fn ui(&mut self, ui: &mut Ui) {
        self.update_skin(ui);

        // Calculate fps averaging over last 10sec
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
        let fps = format!("FPS: {}", self.fps);
        let size = ui.calc_size(&fps);
        let pos = self.position.relative(size, screen(), vec2(0., 0.));
        widgets::Label::new(fps).position(pos).ui(ui);
        ui.pop_skin();
    }
}
