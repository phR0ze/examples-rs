//! Demonstrating button alignment
//! * Align examples of the 10 permutations
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "button align".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::dark().layout(|x| x.align(Align::Center).margins(0., 0., 100., 0.));
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);

    loop {
        clear_background(WHITE);

        let mut p1 =
            Panel::new("p1").layout(|x| x.size_f().padding_all(30.).margins_all(10.)).frame(|x| x.fill(BLACK));

        let align = vec![
            Align::Center,
            Align::CenterBottom,
            Align::CenterTop,
            Align::LeftBottom,
            Align::LeftCenter,
            Align::LeftTop,
            Align::RightBottom,
            Align::RightCenter,
            Align::RightTop,
            Align::Absolute(175., 150.),
        ];
        for i in 0..=9 {
            p1.append(
                Button::icon(format!("B{}", i), format!("B{}", i), icon)
                    .layout(|x| x.size_s(100., 100.).align(align[i]))
                    .frame(|x| x.fill(GRAY)),
            );
        }

        p1.show();
        fps.show();

        next_frame().await
    }
}
