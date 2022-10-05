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
    let builder = ButtonBuilder::new().label_font_size(15.0).background_color(GRAY).padding(50., 50., 10., 10.);

    let mut fps = Fps::new().with_font_color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        builder
            .build("Left Top")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::LeftTop(None))
            .ui(&mut *root_ui(), screen(), offset(0., 50.));

        builder
            .build("Left Center")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::LeftCenter(None))
            .ui(&mut *root_ui(), screen(), offset(0., 100.));

        builder
            .build("Left Bottom")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::LeftBottom(None))
            .ui(&mut *root_ui(), screen(), offset(0., 150.));

        builder
            .build("Right Top")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::RightTop(None))
            .ui(&mut *root_ui(), screen(), offset(0., 200.));

        builder
            .build("Right Center")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::RightCenter(None))
            .ui(&mut *root_ui(), screen(), offset(0., 250.));

        builder
            .build("Right Bottom")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::RightBottom(None))
            .ui(&mut *root_ui(), screen(), offset(0., 300.));

        builder
            .build("Center Top")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::CenterTop(None))
            .ui(&mut *root_ui(), screen(), offset(0., 350.));

        builder
            .build("Center")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::Center(None))
            .ui(&mut *root_ui(), screen(), offset(0., 400.));

        builder
            .build("Center Bottom")
            .with_position(Position::CenterTop(None))
            .with_label_position(Position::CenterBottom(None))
            .ui(&mut *root_ui(), screen(), offset(0., 450.));

        next_frame().await
    }
}
