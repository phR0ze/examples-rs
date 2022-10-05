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

fn skin_with_color(color: Color) -> Skin {
    let button_style = root_ui().style_builder().color(color).color_hovered(color).color_clicked(color).build();
    Skin { button_style, ..root_ui().default_skin() }
}

#[macroquad::main(main_conf)]
async fn main() {
    let blue_skin = skin_with_color(BLUE);
    let gray_skin = skin_with_color(GRAY);
    let green_skin = skin_with_color(GREEN);
    let red_skin = skin_with_color(RED);

    let mut fps = Fps::new().with_position(Position::Center(None));

    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        // Handle positional offset of containing widget for inner widget
        root_ui().push_skin(&gray_skin);
        let size1 = Size::Percent(0.95, 0.40).relative(screen(), None);
        let pos1 = Position::CenterTop(rect(0., 0., 10., 0.)).relative(size1, screen(), None);
        widgets::Button::new("").size(size1).position(pos1).ui(&mut *root_ui());
        root_ui().pop_skin();

        root_ui().push_skin(&blue_skin);
        let size2 = Size::Percent(0.75, 0.75).relative(size1, None);
        let pos2 = Position::Center(None).relative(size2, size1, Some(pos1));
        widgets::Button::new("").size(size2).position(pos2).ui(&mut *root_ui());
        root_ui().pop_skin();

        root_ui().push_skin(&green_skin);
        let size3 = Size::Percent(0.75, 0.75).relative(size2, None);
        let pos3 = Position::Center(None).relative(size3, size2, Some(pos2));
        widgets::Button::new("").size(size3).position(pos3).ui(&mut *root_ui());
        root_ui().pop_skin();

        root_ui().push_skin(&red_skin);
        let size4 = Size::Percent(0.75, 0.75).relative(size3, None);
        let pos4 = Position::Center(None).relative(size4, size3, Some(pos3));
        widgets::Button::new("").size(size4).position(pos4).ui(&mut *root_ui());
        root_ui().pop_skin();

        Label::new("relative to screen with offset").ui(&mut *root_ui(), size4, pos4);

        // No need for positional offset of containing widget
        let size5 = Size::Percent(0.95, 0.40).relative(screen(), None);
        let pos5 = Position::CenterBottom(rect(0., 0., 0., 10.)).relative(size5, screen(), None);
        widgets::Group::new(hash!(), size5).position(pos5).ui(&mut *root_ui(), |ui| {
            ui.push_skin(&gray_skin);
            let size6 = Size::Percent(1., 1.).relative(size5, None);
            let pos6 = Position::Center(None).relative(size6, size5, None);
            widgets::Button::new("").size(size6).position(pos6).ui(ui);
            ui.pop_skin();

            ui.push_skin(&blue_skin);
            let size7 = Size::Percent(0.75, 0.75).relative(size5, None);
            let pos7 = Position::Center(None).relative(size7, size5, None);
            widgets::Button::new("").size(size7).position(pos7).ui(ui);
            ui.pop_skin();

            ui.push_skin(&green_skin);
            let size8 = Size::Percent(0.75, 0.75).relative(size7, None);
            let pos8 = Position::Center(None).relative(size8, size7, Some(pos7));
            widgets::Button::new("").size(size8).position(pos8).ui(ui);
            ui.pop_skin();

            ui.push_skin(&red_skin);
            let size9 = Size::Percent(0.75, 0.75).relative(size8, None);
            let pos9 = Position::Center(None).relative(size9, size8, Some(pos8));
            widgets::Button::new("").size(size9).position(pos9).ui(ui);
            ui.pop_skin();

            Label::new("relative to group with NO offset").ui(ui, size5, origin());
        });

        next_frame().await
    }
}
