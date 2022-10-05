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
    let mut fps = Fps::new().with_font_color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        let mut button = Button::new("Settings")
            .with_background_color(GRAY)
            .with_position(Position::LeftTop(rect(0., 50., 0., 0.)))
            .with_size(Size::three_quarter_width())
            .with_label_position(Position::LeftCenter(rect(40.0, 0., 0., 0.)));
        button.ui(&mut *root_ui(), screen(), None);
        // if button.activated() {
        //     draw_rectangle(100., 100., 50., 50., RED);
        // }

        next_frame().await
    }
}
