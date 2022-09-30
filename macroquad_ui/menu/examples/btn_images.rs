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
    let font_htowert = include_bytes!("../assets/HTOWERT.TTF");
    let menu_bg = Image::from_file_with_format(include_bytes!("../assets/menu_bg.png"), None);
    let entry_bg = Image::from_file_with_format(include_bytes!("../assets/entry_bg.png"), None);
    let entry_clk_bg = Image::from_file_with_format(include_bytes!("../assets/entry_clk_bg.png"), None);
    let mut menu = Menu::new()
        .with_size(Size::Custom(250.0, 250.0))
        .with_background(menu_bg)
        .with_position(Position::Center(None))
        .with_padding(20.0, 20.0, 20.0, 20.0)
        .with_entry_font(font_htowert)
        .with_entry_font_color(Color::from_rgba(180, 180, 100, 255))
        .with_entry_images(entry_bg, entry_clk_bg)
        .with_entry_padding(50., 50., 10., 10.)
        .with_entry_position(Position::CenterTop(None))
        .add_entry("Play")
        .add_entry("Settings")
        .add_entry("Quit");

    loop {
        clear_background(BLACK);

        menu.ui(&mut *root_ui());

        next_frame().await
    }
}
