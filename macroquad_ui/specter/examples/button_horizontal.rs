//! Demonstrating full screen horizontal layout with margin and spacing
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "horizontal".to_string(),
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
        .layout(|x| x.spacing(spacing).padding_all(20.).margins(0., 0., 50., 0.))
        .add(Button::icon("B1", "B1", icon).frame(|x| x.fill(DARKGRAY)))
        .add(Button::icon("B2", "B2", icon).frame(|x| x.fill(RED)))
        .add(Button::icon("B3", "B3", icon).frame(|x| x.fill(BLUE)))
        .add(Button::icon("B4", "B4", icon).frame(|x| x.fill(GREEN)));
    loop {
        clear_background(BLACK);
        fps.show();
        let res = panel.show();

        for x in res.items.iter() {
            if x.activated {
                let widget = panel.get_as::<Button>(&x.id).unwrap();
                let (pos, _) = widget.shape();
                draw_text(&x.id, pos.x + 20., pos.y + 100., 30., widget.get_frame().fill.unwrap());
            }
        }
        next_frame().await
    }
}
