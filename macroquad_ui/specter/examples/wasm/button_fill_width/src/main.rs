//! Demonstrating
//! * percentage size
//! * fill width
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "fill width".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::dark();
    let icon = Texture2D::from_file_with_format(include_bytes!("../../../../assets/options_icon.png"), None);

    let spacing = 10.;
    let mut panel = Panel::vert(id!())
        .layout(|x| x.size_p(0.75, 1.0).spacing(spacing).padding_all(20.).margins(0., 0., 50., 0.).fill_w())
        .add(Button::icon("Button1", "Button1", icon).frame(|x| x.fill(DARKGRAY)))
        .add(Button::icon("Button2", "Button2", icon).frame(|x| x.fill(RED)))
        .add(Button::icon("Button3", "Button3", icon).frame(|x| x.fill(BLUE)))
        .add(Button::icon("Button4", "Button4", icon).frame(|x| x.fill(GREEN)))
        .add(Button::icon("Button5", "Button5", icon).frame(|x| x.fill(ORANGE)))
        .add(Button::icon("Button6", "Button6", icon).frame(|x| x.fill(YELLOW)))
        .add(Button::icon("Button7", "Button7", icon).frame(|x| x.fill(BROWN)))
        .add(Button::icon("Button8", "Button8", icon).frame(|x| x.fill(PURPLE)))
        .add(Button::icon("Button9", "Button9", icon).frame(|x| x.fill(PINK)));
    loop {
        clear_background(BLACK);
        fps.show();
        let res = panel.show();

        for x in res.items.iter() {
            if x.activated {
                let widget = panel.get_as::<Button>(&x.id).unwrap();
                let (pos, _) = widget.shape();
                draw_text(&x.id, pos.x + 320., pos.y + 27., 35., widget.get_frame().fill.unwrap());
            }
        }
        next_frame().await
    }
}
