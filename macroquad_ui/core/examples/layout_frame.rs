//! Demonstrating layouts nested inside other layouts
use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "layout".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::new().color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        Panel::new(id!()).with_fill(GRAY).show(&mut *root_ui(), &mut layout1, |_, _| {});

        //let mut layout1 = Layout::root().with_vert().with_fill().with_margin(10., 10., 50., 10.);

        // let mut layout2 = layout1.nest().with_fill().with_size_p(0.75, 1.0);
        // Panel::new(gid!()).with_fill(BLUE).show(&mut *root_ui(), &mut layout2, |_, _| {});

        // let mut layout3 = layout2.nest().with_fill().with_size_p(0.35, 1.0);
        // Panel::new(gid!()).with_fill(GREEN).show(&mut *root_ui(), &mut layout3, |_, _| {});

        next_frame().await
    }
}
