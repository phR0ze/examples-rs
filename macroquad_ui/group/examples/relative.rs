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
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        Group::new(hash!()).with_size(Size::Static(300., 300.)).ui(
            &mut *root_ui(),
            Size::screen(),
            |ui, container| {
                // widgets::Button::new("button 1").ui(ui);
                // widgets::Button::new("button 2").ui(ui);
                // widgets::Button::new("button 3").ui(ui);
                // widgets::Button::new("button 4").ui(ui);
                Group::new(hash!()).with_size(Size::ThreeQuarter).with_background_color(BLUE).ui(
                    ui,
                    container,
                    |ui, container| {
                        widgets::Button::new("button 1").ui(ui);
                        widgets::Button::new("button 2").ui(ui);
                        widgets::Button::new("button 3").ui(ui);
                        widgets::Button::new("button 4").ui(ui);
                    },
                );
            },
        );

        next_frame().await
    }
}
