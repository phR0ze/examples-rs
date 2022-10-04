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
    let columns = 4;
    let spacing = 10.0;
    let w = screen_width() / columns as f32 - spacing - (spacing / columns as f32);
    let red = GroupBuilder::new().size(Size::Static(w, w)).background_color(RED).draggable(true);
    let blue = GroupBuilder::new().size(Size::Static(w, w)).background_color(BLUE).draggable(true);

    // Track groups outside main look for click persistence
    let mut groups = vec![];
    for i in 0..columns {
        for j in 0..columns {
            let id = format!("{},{}", i, j);
            let pos = Position::Static(j as f32 * (w + spacing) + spacing, i as f32 * (w + spacing) + spacing);
            groups.push(blue.build(id).with_position(pos));
        }
    }

    let mut fps = Fps::new().with_position(Position::LeftBottom(rect(10., 0., 0., 0.)));
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        for group in groups.iter_mut() {
            let drag = group.ui(&mut *root_ui(), Size::screen(), |_, _, _| {});
            if group.toggle() {
                group.background_color(RED);
            }
            if let Drag::Dropped(pos, _) = drag {
                group.position(pos);
            }
        }

        next_frame().await
    }
}
