//! Fps provides a simple frames per second widget to be displayed for debug purposes.
//! * Fps value is averaged over the last 10 seconds for a smoother appearance
use crate::{layout::Layout, prelude::Label};
use macroquad::{prelude::*, ui::Ui};
use std::time::Instant;

const FPS_ID: &'static str = "fps";

pub struct Fps {
    fps: u16,       // last calculated frames per second
    frames: u64,    // count the frames until the next second
    start: Instant, // time to start tracking from
    label: Label,   // label to draw
}

impl Fps {
    /// Create a new Fps light instance
    pub fn new() -> Fps {
        Fps {
            fps: 0,
            frames: 0,
            start: Instant::now(),
            label: Label::new("")
                .with_id(FPS_ID)
                .with_font_color(BLACK)
                .with_layout(|x| x.with_margins(10., 0., 0., 0.)),
        }
    }

    /// Create a new Fps dark instance
    pub fn dark() -> Fps {
        Fps::new().color(WHITE)
    }

    /// Set the font color to use
    pub fn color(self, color: Color) -> Self {
        Fps {
            label: self.label.with_font_color(color),
            ..self
        }
    }

    /// Return the fps right now
    pub fn now(&self) -> u16 {
        self.fps
    }

    /// Set layout to use
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            label: self.label.with_layout(f),
            ..self
        }
    }

    // Update the skin
    pub fn ui(&mut self, ui: &mut Ui) {
        self.label.ui(ui);
    }

    /// Draw the frames per second as directed
    pub fn show(&mut self, ui: &mut Ui, layout: Option<&Layout>) {
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

        // Update label and show
        self.label.set_text(format!("FPS: {}", self.fps));
        self.label.show(ui, layout);
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(vec2(2., 2.), vec2(2., 2.));
    }
}
