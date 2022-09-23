use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};

mod group;
mod menu;
mod position;
mod utils;
use menu::{Menu, MenuEntry, MenuStyle};
use utils::*;

pub struct Resources {
    menu_style: MenuStyle,
}
impl Resources {
    pub fn load() -> Self {
        // Load assets from app data
        let font_htowert = include_bytes!("../assets/HTOWERT.TTF");
        //let menu_bg = Image::from_file_with_format(include_bytes!("../assets/menu_bg.png"), None);
        //let entry_bg = Image::from_file_with_format(include_bytes!("../assets/entry_bg.png"), None);
        //let entry_hov_bg = Image::from_file_with_format(include_bytes!("../assets/entry_hov_bg.png"), None);
        //let entry_clk_bg = Image::from_file_with_format(include_bytes!("../assets/entry_clk_bg.png"), None);

        Resources { menu_style: MenuStyle::new().entry_font(font_htowert) }
    }
}

fn main_conf() -> Conf {
    Conf {
        window_title: "mqui_menu".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    // Note: it is critical that resources and skins are loaded and configured
    // outside the main loop, else you'll get flickering and odd ui behavior.
    let resources = Resources::load();
    let menu = Menu::new(
        hash!("menu"),
        scale_vec2(250., 250.),
        &[MenuEntry::new("Play1"), MenuEntry::new("Settings1"), MenuEntry::new("Quit1")],
        resources.menu_style.clone(),
    );

    loop {
        clear_background(BLACK);

        menu.ui(&mut *root_ui());

        next_frame().await
    }
}
