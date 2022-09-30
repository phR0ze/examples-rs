use mqui_menu::prelude::*;

mod titlebar;
use titlebar::TitleBar;

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
    let mut titlebar = TitleBar::new("Title Bar");

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
