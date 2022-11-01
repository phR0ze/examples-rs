//! Demonstrating sub component alignment with labels in a button
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
    loop {
        clear_background(BLACK);

        let layout = Layout::new(id!()).size_f();
        Button::new("Left Top")
            .color(GRAY)
            .label_size(20.)
            .layout(|x| x.mode(Mode::Align).align(Align::LeftTop).size_s(120., 50.))
            .label_layout(|x| x.align(Align::LeftTop))
            .show(&mut *root_ui(), Some(&layout));
        Button::new("Center Top")
            .color(GRAY)
            .label_size(20.)
            .layout(|x| x.mode(Mode::Align).align(Align::CenterTop).size_s(120., 50.))
            .label_layout(|x| x.align(Align::CenterTop))
            .show(&mut *root_ui(), Some(&layout));
        Button::new("Right Top")
            .color(GRAY)
            .label_size(20.)
            .layout(|x| x.mode(Mode::Align).align(Align::RightTop).size_s(120., 50.))
            .label_layout(|x| x.align(Align::RightTop))
            .show(&mut *root_ui(), Some(&layout));

        next_frame().await
    }
}

//         builder
//             .build("Left Top")
//             .with_position(Align::CenterTop(None))
//             .with_label_position(Align::LeftTop(None))
//             .ui(&mut *root_ui(), screen(), offset(0., 50.));

//         builder
//             .build("Left Center")
//             .with_position(Align::CenterTop(None))
//             .with_label_position(Align::LeftCenter(None))
//             .ui(&mut *root_ui(), screen(), offset(0., 100.));

//         builder
//             .build("Left Bottom")
//             .with_position(Align::CenterTop(None))
//             .with_label_position(Align::LeftBottom(None))
//             .ui(&mut *root_ui(), screen(), offset(0., 150.));

//         builder
//             .build("Right Top")
//             .with_position(Align::CenterTop(None))
//             .with_label_position(Align::RightTop(None))
//             .ui(&mut *root_ui(), screen(), offset(0., 200.));

//         builder
//             .build("Right Center")
//             .with_position(Align::CenterTop(None))
//             .with_label_position(Align::RightCenter(None))
//             .ui(&mut *root_ui(), screen(), offset(0., 250.));

//         builder
//             .build("Right Bottom")
//             .with_position(Align::CenterTop(None))
//             .with_label_position(Align::RightBottom(None))
//             .ui(&mut *root_ui(), screen(), offset(0., 300.));

//         builder
//             .build("Center Top")
//             .with_position(Align::CenterTop(None))
//             .with_label_position(Align::CenterTop(None))
//             .ui(&mut *root_ui(), screen(), offset(0., 350.));

//         builder.build("Center").with_position(Align::CenterTop(None)).with_label_position(Align::Center(None)).ui(
//             &mut *root_ui(),
//             screen(),
//             offset(0., 400.),
//         );

//         builder
//             .build("Center Bottom")
//             .with_position(Align::CenterTop(None))
//             .with_label_position(Align::CenterBottom(None))
//             .ui(&mut *root_ui(), screen(), offset(0., 450.));

//         next_frame().await
//     }
// }
