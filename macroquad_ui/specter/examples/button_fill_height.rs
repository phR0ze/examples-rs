//! Demonstrating
//! * percentage size
//! * fill height
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "fill height".to_string(),
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
    let mut panel = Panel::horz(id!())
        .layout(|x| x.size_p(0.65, 0.85).spacing(spacing).padding_all(5.).margins(0., 0., 50., 0.).fill_h())
        .add(Button::icon("Button1", "B1", icon).frame(|x| x.fill(DARKGRAY)))
        .add(Button::icon("Button2", "B2", icon).frame(|x| x.fill(RED)))
        .add(Button::icon("Button3", "B3", icon).frame(|x| x.fill(BLUE)));
    loop {
        clear_background(BLACK);
        fps.show();
        let res = panel.show();

        for (i, x) in res.items.iter().enumerate() {
            if x.activated {
                let widget = panel.get_as::<Button>(&x.id).unwrap();
                let (pos, _) = widget.shape();
                draw_text(&x.id, 320., pos.y + 27. + (50. * i as f32), 35., widget.get_frame().fill.unwrap());
            }
        }
        next_frame().await
    }
}
