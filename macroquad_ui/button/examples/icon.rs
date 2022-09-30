use button::prelude::*;
use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "button".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut button =
        Button::icon("Settings", icon).with_position(Position::LeftTop(None)).with_background_color(GRAY);

    loop {
        clear_background(BLACK);

        button.ui(&mut *root_ui(), Size::screen());
        if button.activated() {
            draw_rectangle(200., 300., 50., 50., RED);
        }

        next_frame().await
    }
}
