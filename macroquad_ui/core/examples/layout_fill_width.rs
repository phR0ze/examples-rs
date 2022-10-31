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
    let mut btn1 = Button::icon("Button1", icon).color(GRAY);
    let mut btn2 = Button::icon("Button2", icon).color(RED);
    let mut btn3 = Button::icon("Button3", icon).color(BLUE);
    let mut btn4 = Button::icon("Button4", icon).color(GREEN);
    let mut btn5 = Button::icon("Button5", icon).color(ORANGE);
    let mut btn6 = Button::icon("Button6", icon).color(YELLOW);
    let mut btn7 = Button::icon("Button7", icon).color(BROWN);
    let mut btn8 = Button::icon("Button8", icon).color(PURPLE);
    let mut btn9 = Button::icon("Button9", icon).color(PINK);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        let layout = Layout::vert("side_menu").size_p(0.75, 1.0).fill_w().spacing(10.).margins(0., 0., 50., 0.);
        btn1.show(&mut *root_ui(), Some(&layout));
        btn2.show(&mut *root_ui(), Some(&layout));
        btn3.show(&mut *root_ui(), Some(&layout));
        btn4.show(&mut *root_ui(), Some(&layout));
        btn5.show(&mut *root_ui(), Some(&layout));
        btn6.show(&mut *root_ui(), Some(&layout));
        btn7.show(&mut *root_ui(), Some(&layout));
        btn8.show(&mut *root_ui(), Some(&layout));
        btn9.show(&mut *root_ui(), Some(&layout));

        let mut pos = 78.;
        if btn1.activated() {
            draw_text("button1", 300., pos, 30., GRAY)
        }
        pos += 34.;
        if btn2.activated() {
            draw_text("button2", 300., pos, 30., RED)
        }
        pos += 34.;
        if btn3.activated() {
            draw_text("button3", 300., pos, 30., BLUE)
        }
        pos += 34.;
        if btn4.activated() {
            draw_text("button4", 300., pos, 30., GREEN)
        }
        pos += 34.;
        if btn5.activated() {
            draw_text("button5", 300., pos, 30., ORANGE)
        }
        pos += 34.;
        if btn6.activated() {
            draw_text("button6", 300., pos, 30., YELLOW)
        }
        pos += 34.;
        if btn7.activated() {
            draw_text("button7", 300., pos, 30., BROWN)
        }
        pos += 34.;
        if btn8.activated() {
            draw_text("button8", 300., pos, 30., PURPLE)
        }
        pos += 34.;
        if btn9.activated() {
            draw_text("button9", 300., pos, 30., PINK)
        }
        next_frame().await
    }
}
