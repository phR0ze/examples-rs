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

    let spacing = 10.;
    let mut panel = Panel::vert(id!())
        .layout(|x| x.spacing(spacing).padding_all(20.).margins(0., 0., 50., 0.))
        .add(Button::icon(id!(), "Button1", icon).frame(|x| x.fill(DARKGRAY)))
        .add(Button::icon(id!(), "Button2", icon).frame(|x| x.fill(RED)))
        .add(Button::icon(id!(), "Button3", icon).frame(|x| x.fill(BLUE)))
        .add(Button::icon(id!(), "Button4", icon).frame(|x| x.fill(GREEN)))
        .add(Button::icon(id!(), "Button5", icon).frame(|x| x.fill(ORANGE)))
        .add(Button::icon(id!(), "Button6", icon).frame(|x| x.fill(YELLOW)))
        .add(Button::icon(id!(), "Button7", icon).frame(|x| x.fill(BROWN)))
        .add(Button::icon(id!(), "Button8", icon).frame(|x| x.fill(PURPLE)))
        .add(Button::icon(id!(), "Button9", icon).frame(|x| x.fill(PINK)));
    loop {
        clear_background(BLACK);
        fps.show();
        let res = panel.show();

        if res.responses[0].activated {
            //let (pos, _) = panel[0].shape();
            //draw_text("button1", pos.x + 350., pos.y + 30., 30., GRAY)
        }
        // if btn2.activated() {
        //     let (pos, _) = btn2.shape();
        //     draw_text("button2", pos.x + 350., pos.y + 30., 30., RED)
        // }
        // if btn3.activated() {
        //     let (pos, _) = btn3.shape();
        //     draw_text("button3", pos.x + 350., pos.y + 30., 30., BLUE)
        // }
        // if btn4.activated() {
        //     let (pos, _) = btn4.shape();
        //     draw_text("button4", pos.x + 350., pos.y + 30., 30., GREEN)
        // }
        // if btn5.activated() {
        //     let (pos, _) = btn5.shape();
        //     draw_text("button5", pos.x + 350., pos.y + 30., 30., ORANGE)
        // }
        // if btn6.activated() {
        //     let (pos, _) = btn6.shape();
        //     draw_text("button6", pos.x + 350., pos.y + 30., 30., YELLOW)
        // }
        // if btn7.activated() {
        //     let (pos, _) = btn7.shape();
        //     draw_text("button7", pos.x + 350., pos.y + 30., 30., BROWN)
        // }
        // if btn8.activated() {
        //     let (pos, _) = btn8.shape();
        //     draw_text("button8", pos.x + 350., pos.y + 30., 30., PURPLE)
        // }
        // if btn9.activated() {
        //     let (pos, _) = btn9.shape();
        //     draw_text("button9", pos.x + 350., pos.y + 30., 30., PINK)
        // }
        next_frame().await
    }
}
