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
    let group_bldr = GroupBuilder::new()
        .size(Size::Percent(0.85, 0.85))
        .position(Position::Center(None))
        .background_color(BLUE);
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        Group::new(gid!())
            .with_size(Size::Percent(0.95, 0.45))
            .with_position(Position::CenterTop(rect(0., 0., 40., 0.)))
            .ui(&mut *root_ui(), Size::screen(), |ui, cont_size| {
                Group::new(gid!())
                    .with_size(Size::Percent(0.85, 0.85))
                    .with_position(Position::Center(None))
                    .with_background_color(BLUE)
                    .ui(ui, cont_size, |ui, cont_size| {
                        Group::new(gid!())
                            .with_size(Size::Percent(0.85, 0.85))
                            .with_position(Position::Center(None))
                            .with_background_color(GREEN)
                            .ui(ui, cont_size, |ui, cont_size| {
                                //
                            });
                    });
            });

        next_frame().await
    }
}
