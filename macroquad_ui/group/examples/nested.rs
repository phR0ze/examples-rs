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

        let group1_size = vec2(300., 300.);
        let group1_pos = vec2(screen_width() - group1_size.x, screen_height() - group1_size.y) / 2.0;
        widgets::Group::new(hash!(), group1_size).position(group1_pos).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            widgets::Button::new("button 2").ui(ui);
            widgets::Button::new("button 3").ui(ui);
            widgets::Button::new("button 4").ui(ui);
            let group2_size = vec2(150., 150.);
            let group2_pos = vec2(group1_size.x - group2_size.x, group1_size.y - group2_size.y) / 2.0;
            widgets::Group::new(hash!(), group2_size).position(group2_pos).ui(ui, |ui| {
                widgets::Button::new("button 1").ui(ui);
                widgets::Button::new("button 2").ui(ui);
                widgets::Button::new("button 3").ui(ui);
                widgets::Button::new("button 4").ui(ui);
            });
        });

        next_frame().await
    }
}
