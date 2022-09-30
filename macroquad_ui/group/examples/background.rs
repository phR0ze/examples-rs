use test::prelude::*;

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

        // background style has no effect
        root_ui().push_skin(&background);
        widgets::Group::new(hash!(), vec2(100., 100.)).position(vec2(250., 100.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });
        root_ui().pop_skin();

        next_frame().await
    }
}
