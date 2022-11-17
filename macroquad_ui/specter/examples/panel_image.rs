//! Demonstrate frame properties
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "panel image".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let image = Texture2D::from_file_with_format(include_bytes!("../assets/menu_bg.png"), None);

    loop {
        clear_background(WHITE);
        Panel::new(id!())
            .layout(|x| x.size_s(200., 200.).align(Align::Center))
            .frame(|x| x.image(image.clone()))
            .show();

        next_frame().await
    }
}
