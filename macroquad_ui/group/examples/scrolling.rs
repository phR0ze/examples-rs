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
    let white_border = {
        let ui = root_ui();
        let group_style = ui.style_builder().color(WHITE).build();
        Skin { group_style, ..ui.default_skin() }
    };

    let disabled_scrollbar = {
        let ui = root_ui();

        // disable group scrolling feature
        let scroll_width = 0.0;
        let scroll_multiplier = 0.0;
        let scrollbar_style = ui.style_builder().color(BLANK).color_hovered(BLANK).color_clicked(BLANK).build();
        let scrollbar_handle_style =
            ui.style_builder().color(BLANK).color_hovered(BLANK).color_clicked(BLANK).build();

        // add group border color to make is easier to see the edges
        let group_style = ui.style_builder().color(WHITE).build();
        Skin {
            group_style,
            scrollbar_style,
            scrollbar_handle_style,
            scroll_width,
            scroll_multiplier,
            ..ui.default_skin()
        }
    };

    loop {
        clear_background(WHITE);

        // Draw a blue rectangle behind the group showing that the scrollbar does not exceed the
        // original specified group size; rather it consumes the group's size.
        draw_rectangle(99., 99., 82., 82., BLUE);

        // group automatically scrolls if content exceeds the original size
        root_ui().push_skin(&white_border);
        widgets::Group::new(hash!(), vec2(80., 80.)).position(vec2(100., 100.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });
        root_ui().pop_skin();

        // Draw a red rectangle behind the group showing that the disabled scrollbar does not
        // change the group's sizing at all
        draw_rectangle(199., 99., 82., 82., RED);

        // disabling scrolling is possible with some style changes
        root_ui().push_skin(&disabled_scrollbar);
        widgets::Group::new(hash!(), vec2(80., 80.)).position(vec2(200., 100.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });
        root_ui().pop_skin();

        next_frame().await
    }
}
