use mqui_menu::prelude::*;

mod titlebar;
use titlebar::{TitleBar, TitleBarStyle};

pub struct Resources {
    title_bar_style: TitleBarStyle,
}
impl Resources {
    pub fn load() -> Self {
        // Load assets from app data
        let font_htowert = include_bytes!("../assets/HTOWERT.TTF");
        let menu_btn = Image::from_file_with_format(include_bytes!("../assets/menu_btn.png"), None);
        let menu_btn_clk = Image::from_file_with_format(include_bytes!("../assets/menu_btn_clk.png"), None);
        let options_btn = Image::from_file_with_format(include_bytes!("../assets/options_btn.png"), None);
        let options_btn_clk = Image::from_file_with_format(include_bytes!("../assets/options_btn_clk.png"), None);

        Resources {
            title_bar_style: TitleBarStyle {
                padding: scale_rect(15., 15., 5., 5.),
                title_font: font_htowert,
                title_font_size: scale(30.) as u16,
                title_font_color: Color::from_rgba(250, 250, 250, 250),
                menu_btn,
                menu_btn_clk,
                options_btn,
                options_btn_clk,
            },
        }
    }
}

fn main_conf() -> Conf {
    Conf {
        window_title: "mqui_title_bar".to_string(),
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
    let mut titlebar = TitleBar::new(hash!("titlebar"), "Title Bar", resources.title_bar_style);
    let menu =
        Menu::menu().add(MenuEntry::new("Play")).add(MenuEntry::new("Settings")).add(MenuEntry::new("Quit"));
    let options =
        Menu::options().add(MenuEntry::new("Play")).add(MenuEntry::new("Settings")).add(MenuEntry::new("Quit"));

    loop {
        clear_background(BLACK);

        titlebar.ui(&mut *root_ui());
        if titlebar.menu() {
            menu.ui(&mut *root_ui());
        }
        if titlebar.options() {
            options.ui(&mut *root_ui());
        }

        next_frame().await
    }
}
