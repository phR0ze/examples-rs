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
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut btn1 = Button::icon("Options", icon).with_background_color(GRAY);
    let mut btn2 = Button::icon("Settings", icon).with_background_color(RED);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        let layout = Layout::vert("side_menu")
            .with_size_p(0.75, 1.0)
            .with_fill_w()
            .with_spacing(10.)
            .with_margins(0., 0., 50., 0.);
        btn1.show(&mut *root_ui(), Some(&layout));
        btn2.show(&mut *root_ui(), Some(&layout));

        // Panel::new(RED).show(&mut *root_ui(), &mut layout, |_, _| {});
        // Panel::new(BLUE).show(&mut *root_ui(), &mut layout, |_, _| {});
        // Panel::new(GREEN).show(&mut *root_ui(), &mut layout, |_, _| {});
        // Panel::new(ORANGE).show(&mut *root_ui(), &mut layout, |_, _| {});
        // Panel::new(YELLOW).show(&mut *root_ui(), &mut layout, |_, _| {});
        // Panel::new(BROWN).show(&mut *root_ui(), &mut layout, |_, _| {});
        // Panel::new(BEIGE).show(&mut *root_ui(), &mut layout, |_, _| {});
        // Panel::new(PURPLE).show(&mut *root_ui(), &mut layout, |_, _| {});
        // Panel::new(PINK).show(&mut *root_ui(), &mut layout, |_, _| {});

        next_frame().await
    }
}
