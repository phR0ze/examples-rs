use core::prelude::*;
use group::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "group".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::dark();
    let mut group1 = Group::new().with_position(Position::RightTop(None));
    let mut group2 = Group::new().with_background_color(WHITE).with_position(Position::LeftBottom(None));

    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        group1.ui(&mut *root_ui(), Size::screen(), |ui, container| {});
        group2.ui(&mut *root_ui(), Size::screen(), |ui, container| {});

        next_frame().await
    }
}
