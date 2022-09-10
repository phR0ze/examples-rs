use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin, Ui},
};

mod group;
mod menu;
mod position;
mod style;
use group::Group;
use menu::{Menu, MenuEntry};
use style::MenuStyle;

pub struct Resources {
    menu_style: MenuStyle,
}
impl Resources {
    pub fn load() -> Self {
        // Load assets from static memory
        let font_htowert = include_bytes!("../assets/HTOWERT.TTF");
        let image_win_bg = Image::from_file_with_format(include_bytes!("../assets/win_bg.png"), None);
        let image_btn_bg = Image::from_file_with_format(include_bytes!("../assets/btn_bg.png"), None);
        let image_btn_hov_bg = Image::from_file_with_format(include_bytes!("../assets/btn_hov_bg.png"), None);
        let image_btn_clk_bg = Image::from_file_with_format(include_bytes!("../assets/btn_clk_bg.png"), None);

        // Menu style configuration
        let menu_style = MenuStyle {
            background: image_win_bg.clone(),
            padding: RectOffset::new(20., 20., 40., 20.),
            spacing: 10.,

            entry_bg: image_btn_bg.clone(),
            entry_clk_bg: image_btn_clk_bg.clone(),
            entry_hov_bg: image_btn_hov_bg.clone(),
            entry_font: font_htowert,
            entry_font_size: 40,
            entry_font_color: Color::from_rgba(180, 180, 100, 255),
            entry_padding: RectOffset::new(0.0, 0.0, 10.0, 10.0),
        };
        Resources { menu_style }
    }
}

#[macroquad::main("mqui_menu")]
async fn main() {
    // Note: it is critical that resources and skins are loaded and configured
    // outside the main loop, else you'll get flickering and odd ui behavior.
    let resources = Resources::load();
    let menu = Menu::new(
        hash!("main_menu"),
        vec2(300., 300.),
        &[
            MenuEntry { title: "Play".to_string() },
            MenuEntry { title: "Options".to_string() },
            MenuEntry { title: "Quit".to_string() },
        ],
        resources.menu_style,
    );

    loop {
        clear_background(WHITE);

        menu.ui(&mut *root_ui());

        next_frame().await
    }
}
