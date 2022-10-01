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
    let mut group1 = Group::new().with_size(Size::Static(300., 300.)).with_position(Position::Center(None));
    let mut group2 = Group::new().with_size(Size::Static(150., 150.)).with_position(Position::Center(None));
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        group1.ui(&mut *root_ui(), Size::screen(), |ui, container| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
            group2.ui(ui, container, |ui, container| {
                widgets::Button::new("button 1").ui(ui);
                widgets::Button::new("button 2").ui(ui);
                widgets::Button::new("button 3").ui(ui);
                widgets::Button::new("button 4").ui(ui);
            });
        });

        next_frame().await
    }
}
