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

        let mut layout1 =
            Layout::root("MAIN".to_string()).with_vert().with_fill().with_margins(10., 10., 50., 10.);
        Panel::new(id!()).with_fill(GRAY).show(&mut *root_ui(), &mut layout1, |_, _| {});

        // let mut layout2 = layout1.alloc_layout().with_fill().with_size_p(0.75, 1.0);
        // Panel::new(gid!(), BLUE).show(&mut *root_ui(), &mut layout2, |_, _| {});

        // let mut layout3 = layout2.alloc_layout().with_fill().with_size_p(0.35, 1.0);
        // Panel::new(gid!(), GREEN).show(&mut *root_ui(), &mut layout3, |_, _| {});

        next_frame().await
    }
}
