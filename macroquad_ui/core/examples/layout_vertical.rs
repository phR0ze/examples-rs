//! Demonstrating full screen vertical layout with margin and spacing
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
    let mut fps = Fps::dark();
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut btn1 = Button::icon("Button1", icon).fill(GRAY);
    let mut btn2 = Button::icon("Button2", icon).fill(RED);
    let mut btn3 = Button::icon("Button3", icon).fill(BLUE);
    let mut btn4 = Button::icon("Button4", icon).fill(GREEN);
    let mut btn5 = Button::icon("Button5", icon).fill(ORANGE);
    let mut btn6 = Button::icon("Button6", icon).fill(YELLOW);
    let mut btn7 = Button::icon("Button7", icon).fill(BROWN);
    let mut btn8 = Button::icon("Button8", icon).fill(PURPLE);
    let mut btn9 = Button::icon("Button9", icon).fill(PINK);
    loop {
        clear_background(BLACK);
        fps.show(&mut *root_ui());

        let spacing = 10.;
        let layout = Layout::vert("side_menu").size_p(0.75, 1.0).spacing(spacing).margins(0., 0., 50., 0.);
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
        pos += 31. + spacing;
        if btn2.activated() {
            draw_text("button2", 300., pos, 30., RED)
        }
        pos += 32. + spacing;
        if btn3.activated() {
            draw_text("button3", 300., pos, 30., BLUE)
        }
        pos += 33. + spacing;
        if btn4.activated() {
            draw_text("button4", 300., pos, 30., GREEN)
        }
        pos += 33. + spacing;
        if btn5.activated() {
            draw_text("button5", 300., pos, 30., ORANGE)
        }
        pos += 35. + spacing;
        if btn6.activated() {
            draw_text("button6", 300., pos, 30., YELLOW)
        }
        pos += 35. + spacing;
        if btn7.activated() {
            draw_text("button7", 300., pos, 30., BROWN)
        }
        pos += 35. + spacing;
        if btn8.activated() {
            draw_text("button8", 300., pos, 30., PURPLE)
        }
        pos += 35. + spacing;
        if btn9.activated() {
            draw_text("button9", 300., pos, 30., PINK)
        }
        next_frame().await
    }
}
