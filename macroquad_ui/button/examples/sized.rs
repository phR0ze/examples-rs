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
        .with_position(Position::LeftCenter(None))
        .with_width(Width::ThreeQuarter(0., 0.))
        .with_padding(0.0, 0.0, 10.0, 10.0)
        .with_label_position(Position::LeftCenter(rect(40.0, 0., 0., 0.)));

    loop {
        clear_background(BLACK);

        button.ui(&mut *root_ui(), Size::screen());
        if button.activated() {
            draw_rectangle(100., 100., 50., 50., RED);
        }

        next_frame().await
    }
}
