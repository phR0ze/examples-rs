//! Style encapsulates and automates the manipulation of styles
//! Menu entries will be automatically sized to fit the given font while taking into
//! account padding
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Id, Skin, Style, Ui},
};

#[derive(Debug, Clone)]
pub struct MenuStyle {
    pub background: Image,   // image to use as the background for the menu
    pub padding: RectOffset, // pad inside menu this much from edges before content is allowed
    pub spacing: f32,        // space to leave between menu entries

    // Entry style
    pub entry_bg: Image,           // background image to use for menu buttons
    pub entry_clk_bg: Image,       // background image to use for clicked menu buttons
    pub entry_hov_bg: Image,       // background image to use for hovered menu buttons
    pub entry_font: &'static [u8], // font to use for button text
    pub entry_font_color: Color,   // font color to use for button text
    pub entry_font_size: u16,      // font size to use for button text
    pub entry_padding: RectOffset, // button inside is padded before allowing content
}

impl MenuStyle {
    pub fn entry(&self) -> Style {
        root_ui()
            .style_builder()
            .background(self.entry_bg.clone())
            .background_hovered(self.entry_hov_bg.clone())
            .background_clicked(self.entry_clk_bg.clone())
            .font(self.entry_font)
            .unwrap()
            .text_color(self.entry_font_color)
            .text_color_hovered(self.entry_font_color)
            .font_size(self.entry_font_size)
            .build()
    }

    // Return entry height based on font size and padding
    pub fn entry_height(&self) -> f32 {
        self.entry_font_size as f32 + self.entry_padding.top + self.entry_padding.bottom
    }

    /// Return entry size based on given content size and entry font size
    pub fn entry_size(&self, content_size: Vec2) -> Vec2 {
        vec2(content_size.x, self.entry_height())
    }

    /// Return entry position based on the given index location and spacing
    pub fn entry_pos(&self, index: usize) -> Vec2 {
        let spacing = if index != 0 && self.spacing > 0. { index as f32 * self.spacing } else { 0. };
        vec2(0.0, index as f32 * self.entry_height() + spacing)
    }
}
