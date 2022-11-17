//! Demonstrating label basics
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "label".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.show();

        Panel::new(id!())
            .layout(|x| x.size_s(100., 100.).align(Align::Center))
            .frame(|x| x.fill(GRAY))
            .add(Label::new("Test").layout(|x| x.align(Align::Center)))
            .show();

        next_frame().await
    }
}
