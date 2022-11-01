//! Demonstrating widget alignment to parent and sub-layout alignment with labels in a button
//! * Also includes the usage of margins with alignment for relative adjustments
use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "button_align".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let builder =
        ButtonBuilder::new().color(GRAY).label_size(20.).layout(|x| x.mode(Mode::Align).size_s(130., 50.));

    loop {
        clear_background(BLACK);

        builder
            .build("Left Top")
            .layout(|x| x.align(Align::LeftTop).margins(5., 0., 5., 0.))
            .label_layout(|x| x.align(Align::LeftTop).margins(5., 0., 5., 0.))
            .show(&mut *root_ui(), None);
        builder
            .build("Center Top")
            .layout(|x| x.align(Align::CenterTop).margins(0., 0., 25., 20.))
            .label_layout(|x| x.align(Align::CenterTop).margins(0., 0., 25., 20.))
            .show(&mut *root_ui(), None);
        builder
            .build("Right Top")
            .layout(|x| x.align(Align::RightTop).margins(0., 5., 5., 0.))
            .label_layout(|x| x.align(Align::RightTop).margins(0., 5., 5., 0.))
            .show(&mut *root_ui(), None);
        builder
            .build("Left Center")
            .layout(|x| x.align(Align::LeftCenter).margins(10., 5., 0., 0.))
            .label_layout(|x| x.align(Align::LeftCenter).margins(10., 5., 0., 0.))
            .show(&mut *root_ui(), None);
        builder
            .build("Center")
            .layout(|x| x.align(Align::Center))
            .label_layout(|x| x.align(Align::Center))
            .show(&mut *root_ui(), None);
        builder
            .build("Right Center")
            .layout(|x| x.align(Align::RightCenter).margins(0., 5., 0., 0.))
            .label_layout(|x| x.align(Align::RightCenter).margins(0., 5., 0., 0.))
            .show(&mut *root_ui(), None);
        builder
            .build("Left Bottom")
            .layout(|x| x.align(Align::LeftBottom).margins(5., 0., 0., 5.))
            .label_layout(|x| x.align(Align::LeftBottom).margins(5., 0., 0., 5.))
            .show(&mut *root_ui(), None);
        builder
            .build("Bottom Center")
            .layout(|x| x.align(Align::CenterBottom).margins(0., 0., 0., 5.))
            .label_layout(|x| x.align(Align::CenterBottom).margins(0., 0., 0., 5.))
            .show(&mut *root_ui(), None);
        builder
            .build("Right Bottom")
            .layout(|x| x.align(Align::RightBottom).margins(0., 5., 0., 5.))
            .label_layout(|x| x.align(Align::RightBottom).margins(0., 5., 0., 5.))
            .show(&mut *root_ui(), None);

        next_frame().await
    }
}
