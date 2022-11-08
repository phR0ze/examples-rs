//! Demonstrating label underlining
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "button_icon".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut button = Button::icon("Settings", icon).fill(GRAY).layout(|x| x.margins(0., 0., 50., 0.));

    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.show(&mut *root_ui(), None);

        button.show(&mut *root_ui(), None);
        if button.activated() {
            draw_rectangle(200., 300., 50., 50., RED);
        }

        next_frame().await
    }
}
