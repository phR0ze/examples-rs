//! Demonstrating full screen vertical layout filling the width of the layout
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

        let mut base_layout = Layout::root().vert_m().fill_w().spacing(10.).padding(10., 10., 60., 10.);

        Panel::new(RED).show(&mut *root_ui(), &mut base_layout);
        Panel::new(BLUE).show(&mut *root_ui(), &mut base_layout);
        Panel::new(GREEN).show(&mut *root_ui(), &mut base_layout);
        Panel::new(ORANGE).show(&mut *root_ui(), &mut base_layout);
        Panel::new(YELLOW).show(&mut *root_ui(), &mut base_layout);
        Panel::new(BROWN).show(&mut *root_ui(), &mut base_layout);
        Panel::new(BEIGE).show(&mut *root_ui(), &mut base_layout);
        Panel::new(PURPLE).show(&mut *root_ui(), &mut base_layout);
        Panel::new(PINK).show(&mut *root_ui(), &mut base_layout);

        next_frame().await
    }
}
