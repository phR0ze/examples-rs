//! Demonstrating visually layout::tests::layout_combination
//! * Linear layouts and Align layouts are used here
//! * Margins and padding are used in various layouts
//! * Overflow control occurs below because the Panel p1 at full screen will be adjusted for padding
//! and margins which will then push it over its parent boundaries and it will be resized.
//! * Expansion occurs here with the row layouts to fit content
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "combo".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::dark().layout(|x| x.align(Align::LeftBottom).margins(15., 0., 0., 10.));

    loop {
        clear_background(WHITE);

        Panel::new(id!())
            .layout(|x| x.mode(Mode::TopToBottom).size_f().spacing(10.).padding_all(30.).margins_all(10.))
            .frame(|x| x.fill(BLACK))
            .add(
                Panel::new(id!())
                    .layout(|x| x.mode(Mode::LeftToRight).align(Align::Center).spacing(10.).padding_all(20.))
                    .frame(|x| x.fill(DARKGRAY))
                    .add(Panel::new(id!()).layout(|x| x.size_s(100., 100.)).frame(|x| x.fill(RED)))
                    .add(Panel::new(id!()).layout(|x| x.size_s(100., 100.)).frame(|x| x.fill(GRAY)))
                    .add(Panel::new(id!()).layout(|x| x.size_s(100., 100.)).frame(|x| x.fill(BLUE))),
            )
            .add(
                Panel::new(id!())
                    .layout(|x| x.mode(Mode::LeftToRight).align(Align::Center).spacing(10.).padding_all(20.))
                    .frame(|x| x.fill(GREEN))
                    .add(Panel::new(id!()).layout(|x| x.size_s(100., 100.)).frame(|x| x.fill(RED)))
                    .add(Panel::new(id!()).layout(|x| x.size_s(100., 100.)).frame(|x| x.fill(GRAY)))
                    .add(Panel::new(id!()).layout(|x| x.size_s(100., 100.)).frame(|x| x.fill(BLUE))),
            )
            .show(&mut *root_ui());

        fps.show(&mut *root_ui(), None);

        next_frame().await
    }
}
