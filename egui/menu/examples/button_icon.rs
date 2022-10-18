use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "button".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut button =
        Button::icon("Settings", icon).with_position(Position::LeftCenter(None)).with_background_color(GRAY);

    let mut fps = Fps::new().with_font_color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Test");
            });
        });
        egui_macroquad::draw();

        button.ui(&mut *root_ui(), screen(), None);
        if button.activated() {
            draw_rectangle(200., 300., 50., 50., RED);
        }

        next_frame().await
    }
}
