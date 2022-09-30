use button::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "mqui_button".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let background = Image::from_file_with_format(include_bytes!("../assets/entry_bg.png"), None);
    let background_clicked = Image::from_file_with_format(include_bytes!("../assets/entry_clk_bg.png"), None);
    let mut button = Button::new("Settings")
        .with_padding(50.0, 50.0, 10.0, 10.0)
        .with_background_images(background, background_clicked)
        .with_font_color(Color::from_rgba(180, 180, 100, 255))
        .with_font_size(50.0);

    loop {
        clear_background(BLACK);

        button.ui(&mut *root_ui(), Size::screen());
        if button.clicked() {
            draw_rectangle(100., 100., 50., 50., RED);
        }

        next_frame().await
    }
}
