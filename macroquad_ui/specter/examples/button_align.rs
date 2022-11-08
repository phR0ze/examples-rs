//! Demonstrating widget alignment to parent and sub-layout alignment with labels in a button
//! * Also includes the usage of margins with alignment for relative adjustments
use specter::prelude::*;

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
    let builder = ButtonBuilder::new()
        .fill(GRAY)
        .label_size(20.)
        .layout(|x| x.with_mode(Mode::Align).with_size_static(130., 50.));

    loop {
        clear_background(BLACK);

        builder
            .build("Left Top")
            .layout(|x| x.with_align(Align::LeftTop).with_margins(5., 0., 5., 0.))
            .label_layout(|x| x.with_align(Align::LeftTop).with_margins(5., 0., 5., 0.))
            .show(&mut *root_ui(), None);
        builder
            .build("Center Top")
            .layout(|x| x.with_align(Align::CenterTop).with_margins(0., 0., 25., 20.))
            .label_layout(|x| x.with_align(Align::CenterTop).with_margins(0., 0., 25., 20.))
            .show(&mut *root_ui(), None);
        builder
            .build("Right Top")
            .layout(|x| x.with_align(Align::RightTop).with_margins(0., 5., 5., 0.))
            .label_layout(|x| x.with_align(Align::RightTop).with_margins(0., 5., 5., 0.))
            .show(&mut *root_ui(), None);
        builder
            .build("Left Center")
            .layout(|x| x.with_align(Align::LeftCenter).with_margins(10., 5., 0., 0.))
            .label_layout(|x| x.with_align(Align::LeftCenter).with_margins(10., 5., 0., 0.))
            .show(&mut *root_ui(), None);
        builder
            .build("Center")
            .layout(|x| x.with_align(Align::Center))
            .label_layout(|x| x.with_align(Align::Center))
            .show(&mut *root_ui(), None);
        builder
            .build("Right Center")
            .layout(|x| x.with_align(Align::RightCenter).with_margins(0., 5., 0., 0.))
            .label_layout(|x| x.with_align(Align::RightCenter).with_margins(0., 5., 0., 0.))
            .show(&mut *root_ui(), None);
        builder
            .build("Left Bottom")
            .layout(|x| x.with_align(Align::LeftBottom).with_margins(5., 0., 0., 5.))
            .label_layout(|x| x.with_align(Align::LeftBottom).with_margins(5., 0., 0., 5.))
            .show(&mut *root_ui(), None);
        builder
            .build("Bottom Center")
            .layout(|x| x.with_align(Align::CenterBottom).with_margins(0., 0., 0., 5.))
            .label_layout(|x| x.with_align(Align::CenterBottom).with_margins(0., 0., 0., 5.))
            .show(&mut *root_ui(), None);
        builder
            .build("Right Bottom")
            .layout(|x| x.with_align(Align::RightBottom).with_margins(0., 5., 0., 5.))
            .label_layout(|x| x.with_align(Align::RightBottom).with_margins(0., 5., 0., 5.))
            .show(&mut *root_ui(), None);

        next_frame().await
    }
}
