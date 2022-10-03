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
    let white_border = {
        let ui = root_ui();
        let group_style = ui.style_builder().color(WHITE).build();
        Skin { group_style, ..ui.default_skin() }
    };

    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

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

        // scrolling disabled with new Group
        Group::new(gid!())
            .with_border_color(WHITE)
            .with_size(Size::Static(80., 80.))
            .with_position(Position::Static(200., 100.))
            .ui(&mut *root_ui(), Size::screen(), |ui, _, _| {
                widgets::Button::new("button 1").ui(ui);
                widgets::Button::new("button 2").ui(ui);
                widgets::Button::new("button 3").ui(ui);
                widgets::Button::new("button 4").ui(ui);
            });
        next_frame().await
    }
}
