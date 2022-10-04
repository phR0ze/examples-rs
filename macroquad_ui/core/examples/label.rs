use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "core".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    loop {
        clear_background(WHITE);

        let size1 = vec2(100., 50.);
        let pos1 = vec2(10., 10.);
        widgets::Group::new(hash!(), size1).position(pos1).ui(&mut *root_ui(), |ui| {
            Label::new("centered").ui(ui, size1, Position::origin());
        });

        next_frame().await
    }
}
