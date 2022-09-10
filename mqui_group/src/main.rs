use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Layout, Skin, Ui},
};

#[macroquad::main("mqui_group")]
async fn main() {
    let skin = {
        let group_style = root_ui().style_builder().margin(RectOffset::new(10., 10., 10., 10.)).build();
        Skin { group_style, ..root_ui().default_skin() }
    };

    loop {
        clear_background(GRAY);

        root_ui().push_skin(&skin);

        // group automatically scrolls if content exceeds the original size
        widgets::Group::new(hash!(), vec2(80., 80.)).position(vec2(20., 20.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
        });
        root_ui().pop_skin();

        widgets::Group::new(hash!(), vec2(80., 80.)).position(vec2(120., 20.)).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
        });

        widgets::Group::new(hash!(), vec2(80., 80.)).position(vec2(220., 20.)).layout(Layout::Horizontal).ui(
            &mut *root_ui(),
            |ui| {
                widgets::Button::new("button 1").ui(ui);
                widgets::Button::new("button 2").ui(ui);
                widgets::Button::new("button 3").ui(ui);
            },
        );

        next_frame().await
    }
}
