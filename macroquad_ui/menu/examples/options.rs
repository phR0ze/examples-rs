use menu::prelude::*;

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
    let mut options = Menu::options().add_entry("Play").add_entry("Settings").add_entry("Quit");

    loop {
        clear_background(BLACK);

        options.ui(&mut *root_ui());

        next_frame().await
    }
}
