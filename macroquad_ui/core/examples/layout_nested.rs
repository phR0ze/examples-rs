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

        let mut layout1 = Layout::root().vert_m().fill().padding(10., 10., 50., 10.);
        Panel::new(GRAY).show(&mut *root_ui(), &mut layout1);

        let mut layout2 = layout1.nest().fill().size_p(0.75, 1.0);
        Panel::new(BLUE).show(&mut *root_ui(), &mut layout2);

        let mut layout3 = layout2.nest().fill().size_p(0.35, 1.0);
        Panel::new(GREEN).show(&mut *root_ui(), &mut layout3);

        next_frame().await
    }
}
