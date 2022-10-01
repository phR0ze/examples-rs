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
    let bg = Image::from_file_with_format(include_bytes!("../assets/background.png"), None);
    let background = {
        let group_style = root_ui()
            .style_builder()
            .color(BLACK)
            .background(bg.clone())
            .background_clicked(bg.clone())
            .background_hovered(bg.clone())
            .build();
        Skin { group_style, ..root_ui().default_skin() }
    };
    let white_border = {
        let ui = root_ui();
        let group_style = ui.style_builder().color(WHITE).build();
        Skin { group_style, ..ui.default_skin() }
    };

    let mut bg_group = Group::new()
        .with_size(Size::Static(200., 200.))
        .with_position(Position::RightCenter(None))
        .with_background(bg.clone());

    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        // no background with MQ group
        draw_rectangle(99., 99., 102., 102., BLUE);
        root_ui().push_skin(&white_border);
        widgets::Group::new(hash!(), vec2(100., 100.)).position(vec2(100., 100.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });
        root_ui().pop_skin();

        // background style has no effect with MQ group
        root_ui().push_skin(&background);
        widgets::Group::new(hash!(), vec2(100., 100.)).position(vec2(250., 100.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });
        root_ui().pop_skin();

        // background solved with new Group
        Group::new().with_size(Size::Static(200., 200.)).with_position(Position::LeftCenter(None)).ui(
            &mut *root_ui(),
            Size::screen(),
            |ui, _| {
                widgets::Button::new("button 1").ui(ui);
                widgets::Button::new("button 2").ui(ui);
                widgets::Button::new("button 3").ui(ui);
                widgets::Button::new("button 4").ui(ui);
            },
        );

        // background solved with new Group
        bg_group.ui(&mut *root_ui(), Size::screen(), |ui, _| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });

        next_frame().await
    }
}
