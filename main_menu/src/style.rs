//! Style encapsulates and automates the manipulation of styles
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Id, Skin, Style, Ui},
};

#[derive(Debug, Clone)]
pub struct MenuStyle {
    pub background: Image,  // image to use as the background for the menu
    pub margin: RectOffset, // menu content is indented this much from edges
    pub spacing: f32,       // space to leave between menu entries

    // Button style
    pub btn_bg: Image,           // background image to use for menu buttons
    pub btn_clk_bg: Image,       // background image to use for clicked menu buttons
    pub btn_hov_bg: Image,       // background image to use for hovered menu buttons
    pub btn_font: &'static [u8], // font to use for button text
    pub btn_font_color: Color,   // font color to use for button text
    pub btn_font_size: u16,      // font size to use for button text
    pub btn_margin: RectOffset,  // button content is indented this much from edges
}

impl MenuStyle {
    pub fn button(&self) -> Style {
        root_ui()
            .style_builder()
            .background(self.btn_bg.clone())
            .margin(self.btn_margin.clone())
            .background_hovered(self.btn_hov_bg.clone())
            .background_clicked(self.btn_clk_bg.clone())
            .font(self.btn_font)
            .unwrap()
            .text_color(self.btn_font_color)
            .font_size(self.btn_font_size)
            .build()
    }

    /// Return button size based on given content size and button font size
    pub fn button_size(&self, content_size: Vec2) -> Vec2 {
        vec2(content_size.x, self.btn_font_size as f32)
    }

    /// Return button position based on the given index location and spacing
    pub fn button_pos(&self, index: usize) -> Vec2 {
        let spacing = if index != 0 && self.spacing > 0. { index as f32 * self.spacing } else { 0. };
        vec2(0.0, index as f32 * self.btn_font_size as f32 + spacing)
    }
}
