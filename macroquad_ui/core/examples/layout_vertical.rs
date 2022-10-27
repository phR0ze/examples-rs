//! Demonstrating full screen veritical layout with margin and spacing
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

        let mut layout1 = Layout::root().with_vert().with_spacing(10.).with_margins(10., 10., 60., 10.);

        Panel::new(RED).show(&mut *root_ui(), &mut layout1, |_, _| {});
        Panel::new(BLUE).show(&mut *root_ui(), &mut layout1, |_, _| {});
        Panel::new(GREEN).show(&mut *root_ui(), &mut layout1, |_, _| {});
        Panel::new(ORANGE).show(&mut *root_ui(), &mut layout1, |_, _| {});
        Panel::new(YELLOW).show(&mut *root_ui(), &mut layout1, |_, _| {});
        Panel::new(BROWN).show(&mut *root_ui(), &mut layout1, |_, _| {});
        Panel::new(BEIGE).show(&mut *root_ui(), &mut layout1, |_, _| {});
        Panel::new(PURPLE).show(&mut *root_ui(), &mut layout1, |_, _| {});
        Panel::new(PINK).show(&mut *root_ui(), &mut layout1, |_, _| {});

        next_frame().await
    }
}
