use button::prelude::*;
use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "mqui_button".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut button = Button::new("Settings")
        .with_position(Position::LeftCenter(None))
        .with_width(Width::ThreeQuarter(0., 0.))
        .with_padding(0.0, 0.0, 10.0, 10.0)
        .with_label_position(Position::LeftCenter(rect(40.0, 0., 0., 0.)));

    let white_border = {
        let ui = root_ui();
        let group_style = ui.style_builder().color(WHITE).build();
        Skin { group_style, ..ui.default_skin() }
    };
    loop {
        clear_background(BLACK);
        root_ui().push_skin(&white_border);

        let size = vec2(300., 300.);
        let pos = vec2((screen_width() - size.x) / 2.0, (screen_height() - size.y) / 2.0);
        widgets::Group::new(hash!(), size).position(pos).ui(&mut *root_ui(), |ui| {
            button.ui(ui, size);
            if button.activated() {
                draw_rectangle(100., 100., 50., 50., RED);
            }
        });

        root_ui().pop_skin();
        next_frame().await
    }
}
