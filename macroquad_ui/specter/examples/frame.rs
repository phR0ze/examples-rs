//! Demonstrate frame properties
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "example-frame".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::new();
    let mut panel = Panel::new(id!());

    loop {
        clear_background(WHITE);

        panel.show_f(&mut *root_ui(), |ui, layout| {
            fps.show(ui, Some(layout));
        });

        next_frame().await
    }
}
