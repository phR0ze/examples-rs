use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "layout".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut button = Button::icon("Settings", icon).with_background_color(GRAY);

    let mut fps = Fps::new().with_font_color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        let mut layout1 = Layout::root().with_vert().with_margin(0., 0., 50., 0.);

        button.show(&mut *root_ui(), &mut layout1);
        if button.activated() {
            draw_rectangle(200., 300., 50., 50., RED);
        }

        next_frame().await
    }
}
