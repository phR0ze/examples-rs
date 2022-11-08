//! Demonstrating layouts nested inside other layouts
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
    let mut menu = Panel::new(id!())
        .with_frame(|x| x.with_fill(GRAY))
        .layout(|x| x.with_size_percent(0.75, 1.0).with_fill_width().with_spacing(10.));
    let mut btn1 = Button::icon("Button1", icon).fill(RED).layout(|x| x.with_margins(0., 0., 10., 0.));
    let mut btn2 = Button::icon("Button2", icon).fill(BLUE);
    let mut btn3 = Button::icon("Button3", icon).fill(GREEN);
    loop {
        clear_background(BLACK);

        menu.show(&mut *root_ui(), None, |ui, layout| {
            btn1.show(ui, Some(layout));
            btn2.show(ui, Some(layout));
            btn3.show(ui, Some(layout));
        });

        // Check button results
        if btn1.activated() {
            let (pos, _) = btn1.shape();
            draw_rectangle(pos.x + 380., pos.y + 2., 40., 40., RED);
        }
        if btn2.activated() {
            let (pos, _) = btn2.shape();
            draw_rectangle(pos.x + 380., pos.y + 2., 40., 40., BLUE);
        }
        if btn3.activated() {
            let (pos, _) = btn3.shape();
            draw_rectangle(pos.x + 380., pos.y + 2., 40., 40., GREEN);
        }

        next_frame().await
    }
}
