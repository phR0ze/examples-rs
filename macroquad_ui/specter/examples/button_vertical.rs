//! Demonstrating full screen vertical layout with margin and spacing
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
        fps.show(&mut *root_ui(), None);

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

        if btn1.activated() {
            let (pos, _) = btn1.shape();
            draw_text("button1", pos.x + 350., pos.y + 30., 30., GRAY)
        }
        if btn2.activated() {
            let (pos, _) = btn2.shape();
            draw_text("button2", pos.x + 350., pos.y + 30., 30., RED)
        }
        if btn3.activated() {
            let (pos, _) = btn3.shape();
            draw_text("button3", pos.x + 350., pos.y + 30., 30., BLUE)
        }
        if btn4.activated() {
            let (pos, _) = btn4.shape();
            draw_text("button4", pos.x + 350., pos.y + 30., 30., GREEN)
        }
        if btn5.activated() {
            let (pos, _) = btn5.shape();
            draw_text("button5", pos.x + 350., pos.y + 30., 30., ORANGE)
        }
        if btn6.activated() {
            let (pos, _) = btn6.shape();
            draw_text("button6", pos.x + 350., pos.y + 30., 30., YELLOW)
        }
        if btn7.activated() {
            let (pos, _) = btn7.shape();
            draw_text("button7", pos.x + 350., pos.y + 30., 30., BROWN)
        }
        if btn8.activated() {
            let (pos, _) = btn8.shape();
            draw_text("button8", pos.x + 350., pos.y + 30., 30., PURPLE)
        }
        if btn9.activated() {
            let (pos, _) = btn9.shape();
            draw_text("button9", pos.x + 350., pos.y + 30., 30., PINK)
        }
        next_frame().await
    }
}
