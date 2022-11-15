//! Demonstrate frame properties
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "frame".to_string(),
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
        let mut ui = &mut *root_ui();
        clear_background(WHITE);
        fps.show(ui, None);

        next_frame().await
    }
}
