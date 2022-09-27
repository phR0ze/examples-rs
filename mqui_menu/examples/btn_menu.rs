use mqui_menu::prelude::*;

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
    let entry_hov_bg = Image::from_file_with_format(include_bytes!("../assets/entry_hov_bg.png"), None);
    let entry_clk_bg = Image::from_file_with_format(include_bytes!("../assets/entry_clk_bg.png"), None);
    let menu = Menu::new()
        .size(Size::Absolute(250.0, 250.0))
        .background(menu_bg)
        .entry_bg(entry_bg)
        .entry_clk_bg(entry_clk_bg)
        .entry_hov_bg(entry_hov_bg)
        .position(Position::Center)
        .entry_font(font_htowert)
        .entry_font_color(Color::from_rgba(180, 180, 100, 255))
        .padding(20.0, 20.0, 20.0, 20.0)
        .add(MenuEntry::new("Play"))
        .add(MenuEntry::new("Settings"))
        .add(MenuEntry::new("Quit"));

    loop {
        clear_background(BLACK);

        menu.ui(&mut *root_ui());

        next_frame().await
    }
}
