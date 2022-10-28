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
    let mut btn1 = Button1::new("Settings")
        .with_background_color(GRAY)
        .with_padding(0., 0., 10., 0.)
        .with_position(Align::LeftTop(None))
        .with_size(Size::three_quarter_width())
        .with_label_position(Align::LeftCenter(rect(10.0, 0., 0., 0.)));
    let mut btn2 = Button1::new("Configuration")
        .with_background_color(GRAY)
        .with_padding(0., 0., 10., 0.)
        .with_position(Align::LeftTop(None))
        .with_size(Size::half_width())
        .with_label_position(Align::LeftCenter(rect(10.0, 0., 0., 0.)));
    let mut fps = Fps::new().with_font_color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        btn1.ui(&mut *root_ui(), screen(), offset(0., 50.));
        if btn1.activated() {
            draw_rectangle(200., 300., 50., 50., RED);
        }
        btn2.ui(&mut *root_ui(), screen(), offset(0., 150.));
        if btn2.activated() {
            draw_rectangle(300., 300., 50., 50., BLUE);
        }

        next_frame().await
    }
}
