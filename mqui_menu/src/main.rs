use macroquad::{prelude::*, ui::root_ui};

mod group;
mod menu;
mod position;
mod size;
mod utils;
use menu::{Menu, MenuEntry};

pub struct Resources {
    font: &'static [u8], // font to use for button text
}
impl Resources {
    pub fn load() -> Self {
        // Load assets from app data
        let font_htowert = include_bytes!("../assets/HTOWERT.TTF");
        //let menu_bg = Image::from_file_with_format(include_bytes!("../assets/menu_bg.png"), None);
        //let entry_bg = Image::from_file_with_format(include_bytes!("../assets/entry_bg.png"), None);
        //let entry_hov_bg = Image::from_file_with_format(include_bytes!("../assets/entry_hov_bg.png"), None);
        //let entry_clk_bg = Image::from_file_with_format(include_bytes!("../assets/entry_clk_bg.png"), None);

        Resources { font: font_htowert }
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
    let resources = Resources::load();
    let menu =
        Menu::menu().add(MenuEntry::new("Play1")).add(MenuEntry::new("Settings1")).add(MenuEntry::new("Quit1"));
    let settings = Menu::settings()
        .add(MenuEntry::new("Play1"))
        .add(MenuEntry::new("Settings1"))
        .add(MenuEntry::new("Quit1"));

    loop {
        clear_background(BLACK);

        //menu.ui(&mut *root_ui());
        settings.ui(&mut *root_ui());

        next_frame().await
    }
}
