//! Demonstrating button icon
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "button icon".to_string(),
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
        Button::icon(id!(), "Settings", icon).frame(|x| x.fill(GRAY)).layout(|x| x.align(Align::Center));

    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.show();

        button.show();
        if button.activated() {
            draw_rectangle(200., 300., 50., 50., RED);
        }

        next_frame().await
    }
}
