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
    let margin = {
        let group_style = root_ui()
            .style_builder()
            .color(WHITE)
            .background_margin(RectOffset::new(10., 10., 10., 10.))
            .margin(RectOffset::new(10., 10., 10., 10.))
            .build();
        Skin { group_style, ..root_ui().default_skin() }
    };
    let white_border = {
        let ui = root_ui();
        let group_style = ui.style_builder().color(WHITE).build();
        Skin { group_style, ..ui.default_skin() }
    };

    loop {
        clear_background(WHITE);

        // Using a blue rectangle behind the group to demonstrate margin effects
        // Offset by 1px and increase size by 2x so we have a surrounding 1px color
        draw_rectangle(99., 99., 102., 102., BLUE);
        root_ui().push_skin(&white_border);
        widgets::Group::new(hash!(), vec2(100., 100.)).position(vec2(100., 100.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });
        root_ui().pop_skin();

        // Using a red rectangle behind the group to demonstrate margin effects
        // Offset by 1px and increase size by 2x so we have a surrounding 1px color
        draw_rectangle(249., 99., 102., 102., RED);
        root_ui().push_skin(&margin);
        widgets::Group::new(hash!(), vec2(100., 100.)).position(vec2(250., 100.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });
        root_ui().pop_skin();

        // margins that work
        Group::new()
            .with_padding(20., 20., 20., 20.)
            .with_size(Size::Custom(150., 150.))
            .with_position(Position::Center(None))
            .ui(&mut *root_ui(), |ui, _| {
                widgets::Button::new("button 1").ui(ui);
                widgets::Button::new("button 2").ui(ui);
                widgets::Button::new("button 3").ui(ui);
                widgets::Button::new("button 4").ui(ui);
            });
        next_frame().await
    }
}
