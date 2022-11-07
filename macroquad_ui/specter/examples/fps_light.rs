//! Demonstrate FPS in top left of the screen
use specter::prelude::*;

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
    let mut fps = Fps::new().layout(|x| x.margins(10., 0., 10., 0.));

    loop {
        clear_background(WHITE);

        fps.show(&mut *root_ui(), None);

        next_frame().await
    }
}
