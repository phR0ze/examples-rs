use core::prelude::*;
use group::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "test".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let grouper = GroupBuilder::new().size(Size::Static(50., 50.)).background_color(BLUE);
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        for i in 1..10 {
            let pos = Position::Static(10., i as f32 * 50. + i as f32 * 10.);
            grouper.build(i.to_string()).with_position(pos).ui(
                &mut *root_ui(),
                Size::screen(),
                |ui, cont_size| {},
            );
        }

        next_frame().await
    }
}
