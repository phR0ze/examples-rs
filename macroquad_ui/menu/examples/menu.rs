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
    let mut menu = Menu::menu().add_entry("Play").add_entry("Settings").add_entry("Quit");

    loop {
        clear_background(BLACK);

        menu.ui(&mut *root_ui());
        if let Some(label) = menu.entry_clicked() {
            println!("Entry: {}", label);
        }

        next_frame().await
    }
}
