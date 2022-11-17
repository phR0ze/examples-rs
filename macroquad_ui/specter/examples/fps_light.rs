//! Demonstrate FPS in top left of the screen
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "example-fps".to_string(),
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

        next_frame().await
    }
}
