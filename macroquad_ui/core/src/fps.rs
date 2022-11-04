//! Fps provides a simple frames per second widget to be displayed for debug purposes.
//! * Fps value is averaged over the last 10 seconds for a smoother appearance

use crate::layout::Layout;
use crate::utils::*;
use macroquad::{
    prelude::*,
    ui::{widgets, Skin, Ui},
};
use std::time::Instant;

const FPS_ID: &'static str = "fps";

pub struct Fps {
    fps: u16,           // last calculated frames per second
    dirty: bool,        // track if the update skin function needs run
    skin: Option<Skin>, // skin cache for frames per second
    frames: u64,        // count the frames until the next second
    start: Instant,     // time to start tracking from
    font_color: Color,  // font color to use
    layout: Layout,     // layout for the widget
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
            layout: Layout::new(FPS_ID),
        }
    }

    /// Create a new Fps dark instance
    pub fn dark() -> Fps {
        Fps::new().color(WHITE)
    }

    /// Set the font color to use
    pub fn color(self, color: Color) -> Self {
        Fps { dirty: true, font_color: color, ..self }
    }

    /// Return the fps right now
    pub fn now(&self) -> u16 {
        self.fps
    }

    /// Set layout to use
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self { layout: f(self.layout), ..self }
    }

    // Update the skin
    fn ui(&mut self, ui: &mut Ui) {
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
    pub fn show(&mut self, ui: &mut Ui) {
        self.ui(ui);

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
        self.layout.set_size(ui.calc_size(&fps));
        let (pos, _) = self.layout.shape();
        widgets::Label::new(fps).position(pos).ui(ui);
        ui.pop_skin();
    }
}
