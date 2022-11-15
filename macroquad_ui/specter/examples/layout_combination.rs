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
    loop {
        clear_background(WHITE);

        let mut p1 = Panel::new("p1")
            .layout(|x| x.mode(Mode::TopToBottom).size_f().spacing(10.).padding_all(30.).margins_all(10.))
            .frame(|x| x.fill(BLACK));
        let mut r1 = Panel::new("p2")
            .layout(|x| x.mode(Mode::LeftToRight).align(Align::Center).spacing(10.).padding_all(20.).parent(&p1))
            .frame(|x| x.fill(DARKGRAY));
        let mut r1c1 = Panel::new("0").layout(|x| x.size_s(100., 100.).parent(&r1)).frame(|x| x.fill(RED));
        let mut r1c2 = Panel::new("1").layout(|x| x.size_s(100., 100.).parent(&r1)).frame(|x| x.fill(GRAY));
        let mut r1c3 = Panel::new("2").layout(|x| x.size_s(100., 100.).parent(&r1)).frame(|x| x.fill(BLUE));

        let mut r2 = Panel::new("p3")
            .layout(|x| x.mode(Mode::LeftToRight).align(Align::Center).spacing(10.).padding_all(20.).parent(&p1))
            .frame(|x| x.fill(GREEN));
        let mut r2c1 = Panel::new("0").layout(|x| x.size_s(100., 100.).parent(&r2)).frame(|x| x.fill(RED));
        let mut r2c2 = Panel::new("1").layout(|x| x.size_s(100., 100.).parent(&r2)).frame(|x| x.fill(GRAY));
        let mut r2c3 = Panel::new("2").layout(|x| x.size_s(100., 100.).parent(&r2)).frame(|x| x.fill(BLUE));

        p1.show(&mut *root_ui());
        r1.show(&mut *root_ui());
        r1c1.show(&mut *root_ui());
        r1c2.show(&mut *root_ui());
        r1c3.show(&mut *root_ui());
        r2.show(&mut *root_ui());
        r2c1.show(&mut *root_ui());
        r2c2.show(&mut *root_ui());
        r2c3.show(&mut *root_ui());

        next_frame().await
    }
}
