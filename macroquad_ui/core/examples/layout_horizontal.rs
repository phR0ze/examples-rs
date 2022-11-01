//! Demonstrating full screen horizontal layout with spacing and margin
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
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut btn1 = Button::icon("Button1", icon).color(GRAY);
    let mut btn2 = Button::icon("Button2", icon).color(RED);
    let mut btn3 = Button::icon("Button3", icon).color(BLUE);

    loop {
        clear_background(BLACK);
        fps.show(&mut *root_ui());

        let layout = Layout::horz("menu").size_f().spacing(10.).margins(0., 0., 50., 0.);
        btn1.show(&mut *root_ui(), Some(&layout));
        btn2.show(&mut *root_ui(), Some(&layout));
        btn3.show(&mut *root_ui(), Some(&layout));

        next_frame().await
    }
}
