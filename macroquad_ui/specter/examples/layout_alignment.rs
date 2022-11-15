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
    loop {
        clear_background(WHITE);

        let mut p1 = Panel::new("p1")
            .with_layout(|x| x.with_size_full().with_padding_all(30.).with_margins_all(10.))
            .with_frame(|x| x.with_fill(BLACK));

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
        let mut shapes = vec![];
        for i in 0..=9 {
            shapes.push(
                Panel::new(format!("{}", i))
                    .with_layout(|x| x.with_size_static(100., 100.).with_align(align[i]).with_parent(&p1.layout()))
                    .with_frame(|x| x.with_fill(GRAY)),
            );
        }

        p1.show(&mut *root_ui());
        for x in shapes.iter_mut() {
            x.show(&mut *root_ui());
        }

        next_frame().await
    }
}
