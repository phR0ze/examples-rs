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
    let mut settings = Button1::new("Settings")
        .with_background_color(GRAY)
        .with_position(Align::LeftCenter(None))
        .with_size(Size::three_quarter_width())
        .with_padding(0.0, 0.0, 10.0, 10.0)
        .with_label_position(Align::LeftCenter(rect(40.0, 0., 0., 0.)));

    let white_border = {
        let ui = root_ui();
        let group_style = ui.style_builder().color(WHITE).build();
        Skin { group_style, ..ui.default_skin() }
    };

    let mut fps = Fps::new().color(WHITE);
    loop {
        clear_background(BLACK);
        fps.show(&mut *root_ui());

        root_ui().push_skin(&white_border);
        let size = vec2(300., 300.);
        let pos = Align::Center(None).relative(size, screen(), None);
        widgets::Group::new(hash!(), size).position(pos).ui(&mut *root_ui(), |ui| {
            settings.ui(ui, size, None);
            if settings.activated() {
                draw_rectangle(100., 100., 50., 50., RED);
            }
        });
        root_ui().pop_skin();

        next_frame().await
    }
}
