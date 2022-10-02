use core::prelude::*;
use group::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "test".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

fn skin_with_color(color: Color) -> Skin {
    let button_style = root_ui().style_builder().color(color).color_hovered(color).color_clicked(color).build();
    Skin { button_style, ..root_ui().default_skin() }
}

#[macroquad::main(main_conf)]
async fn main() {
    let blue_skin = skin_with_color(BLUE);
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        for i in 1..10 {
            for j in 1..7 {
                let size = vec2(50., 50.);
                let pos = Position::Static(j as f32 * (50. + 10.), i as f32 * (50. + 10.)).relative(
                    size,
                    Size::screen(),
                    None,
                );
                root_ui().push_skin(&blue_skin);

                // Wrapping the buttons in a group dropped the fps down in half to 30 fps
                widgets::Group::new(hash!(i.to_string()), size).position(pos).ui(&mut *root_ui(), |ui| {
                    // 70 buttons allow for 60 fps
                    widgets::Button::new("").size(size).position(vec2(0., 0.)).ui(ui);
                });
                root_ui().pop_skin();
            }
        }

        next_frame().await
    }
}
