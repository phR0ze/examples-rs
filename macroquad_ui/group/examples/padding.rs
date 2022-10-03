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

fn skin_with_color(color: Color) -> Skin {
    let button_style = root_ui().style_builder().color(color).color_hovered(color).color_clicked(color).build();
    Skin { button_style, ..root_ui().default_skin() }
}

#[macroquad::main(main_conf)]
async fn main() {
    let blue_skin = skin_with_color(BLUE);
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        Group::new(gid!())
            .with_size(Size::Percent(0.95, 0.45))
            .with_padding(50., 50., 50., 50.)
            .with_position(Position::CenterTop(rect(0., 0., 40., 0.)))
            .ui(&mut *root_ui(), Size::screen(), |ui, size, pos| {
                ui.push_skin(&blue_skin);
                let size1 = Size::Percent(1., 1.).relative(size);
                let pos1 = Position::Center(None).relative(size1, size, Some(pos));
                widgets::Button::new("blue is group content offset by padding").size(size1).position(pos1).ui(ui);
                ui.pop_skin();
            });

        next_frame().await
    }
}
