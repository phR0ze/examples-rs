//! Demonstrating full screen horizontal layout
use specter::prelude::*;

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
    let mut btn1 = Button::icon("B1", icon).fill(GRAY);
    let mut btn2 = Button::icon("B2", icon).fill(RED);
    let mut btn3 = Button::icon("B3", icon).fill(BLUE);
    loop {
        clear_background(BLACK);

        let spacing = 10.;
        let layout = Layout::horz("side_menu").with_size_full().with_spacing(spacing);
        btn1.show(&mut *root_ui(), Some(&layout));
        btn2.show(&mut *root_ui(), Some(&layout));
        btn3.show(&mut *root_ui(), Some(&layout));

        let (pos, _) = btn1.shape();
        if btn1.activated() {
            draw_text("button1", pos.x + 350., pos.y + 30., 30., GRAY)
        }
        if btn2.activated() {
            draw_text("button2", pos.x + 350., pos.y + 60., 30., RED)
        }
        if btn3.activated() {
            draw_text("button3", pos.x + 350., pos.y + 90., 30., BLUE)
        }
        next_frame().await
    }
}
