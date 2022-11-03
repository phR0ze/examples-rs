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
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut menu = Panel::new("menu").fill(GRAY).layout(|x| x.size_p(0.75, 1.0));
    let mut btn1 = Button::icon("Settings", icon).fill(RED);
    menu.alloc(&btn1);
    loop {
        clear_background(BLACK);

        menu.show(&mut *root_ui(), None, |ui, layout| {
            btn1.show(ui, Some(layout));
        });
        if btn1.activated() {
            draw_rectangle(200., 300., 50., 50., RED);
        }

        next_frame().await
    }
}
