use test::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "simple".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    //let icon = Image::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);

    loop {
        clear_background(WHITE);

        root_ui().label(None, "hello megaui");
        if root_ui().button(None, "Push me") {
            println!("pushed");
        }

        next_frame().await
    }
}
