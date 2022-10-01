use button::prelude::*;
use core::prelude::*;

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
    let mut button = Button::new("Settings")
        .with_background_color(GRAY)
        .with_padding(50.0, 50.0, 10.0, 10.0)
        .with_label_position(Position::RightCenter(None));

    let mut fps = Fps::new().with_font_color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        button.ui(&mut *root_ui(), Size::screen());
        if button.activated() {
            draw_rectangle(100., 100., 50., 50., RED);
        }

        next_frame().await
    }
}