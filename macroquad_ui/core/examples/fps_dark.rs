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
    let mut fps = Fps::dark().with_position(Align::RightTop(rect(0., 20., 10., 0.)));

    loop {
        clear_background(BLACK);

        fps.ui(&mut *root_ui());

        next_frame().await
    }
}
