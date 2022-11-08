//! Demonstrate FPS widget in top right corner of the screen with margins pushing it away from the
//! corner and using a dark theme
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
    let mut fps = Fps::dark().layout(|x| x.with_align(Align::RightTop).with_margins(0., 20., 10., 0.));

    loop {
        clear_background(BLACK);

        fps.show(&mut *root_ui(), None);

        next_frame().await
    }
}
