//! Demonstrating visually layout::tests::layout_alignment
//! * Align examples of the 10 permutations
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "align".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::dark().layout(|x| x.align(Align::Center).margins(0., 0., 100., 0.));

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
                Panel::new(format!("{}", i))
                    .layout(|x| x.size_s(100., 100.).align(align[i]))
                    .frame(|x| x.fill(GRAY)),
            );
        }

        p1.show(&mut *root_ui());
        fps.show(&mut *root_ui(), None);

        next_frame().await
    }
}
