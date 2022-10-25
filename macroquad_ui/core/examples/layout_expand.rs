//! Demonstrating layout that expands to the content and properties given
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
    let mut fps = Fps::new().with_font_color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        let mut layout1 = Layout::horz().with_expand().with_spacing(10.).with_margin(10., 10., 60., 10.);

        Panel::new(GRAY).show(&mut *root_ui(), &mut layout1, |_, _| {});

        next_frame().await
    }
}
