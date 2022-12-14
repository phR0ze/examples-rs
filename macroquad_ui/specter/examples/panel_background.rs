//! Demonstrate frame properties
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "panel background".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let image = Texture2D::from_file_with_format(include_bytes!("../assets/entry_bg.png"), None);
    let image_clk = Texture2D::from_file_with_format(include_bytes!("../assets/entry_clk_bg.png"), None);
    let mut fps = Fps::new();

    loop {
        clear_background(WHITE);
        fps.show();

        let button = Panel::new(id!())
            .layout(|x| x.size_s(200., 50.).align(Align::LeftTop).margins(20., 0., 100., 0.))
            .frame(|x| x.image(image).image_clk(image_clk))
            .show();
        if button.clicked {
            println!("image clicked");
        }

        let button = Panel::new(id!())
            .layout(|x| x.size_s(200., 50.).align(Align::LeftTop).margins(20., 0., 200., 0.))
            .frame(|x| x.fill(GRAY).fill_clk(BLUE).fill_hov(RED))
            .show();
        if button.clicked {
            println!("color clicked");
        }

        let button = Panel::new(id!())
            .layout(|x| x.size_s(200., 50.).align(Align::LeftTop).margins(20., 0., 300., 0.))
            .frame(|x| x.fill(RED))
            .show();
        if button.clicked {
            println!("disabled");
        }

        next_frame().await
    }
}
