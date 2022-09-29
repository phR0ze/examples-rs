use mqui_button::prelude::*;

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
    let icon = Image::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut button = Button::new("Settings")
        .background_color(GRAY)
        .position(Position::LeftCenter(None))
        .width(Width::ThreeQuarter(0., 0.))
        .icon_image(icon)
        .icon_position(Position::LeftCenter(rect(20., 0., 0., 0.)))
        .label_position(Position::LeftCenter(rect(80., 0., 3., 0.)));

    loop {
        clear_background(BLACK);

        button.ui(&mut *root_ui());
        if button.toggle() {
            draw_rectangle(100., 100., 50., 50., RED);
        }

        next_frame().await
    }
}
