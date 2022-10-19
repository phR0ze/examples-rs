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
    let mut pixels_per_point: Option<f32> = None;

    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        // Draw egui components
        egui_macroquad::ui(|ctx| {
            // Get current pixels per point
            if pixels_per_point.is_none() {
                pixels_per_point = Some(ctx.pixels_per_point());
            }
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("The triangle is being painted using ");
                    ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
                    ui.label(" (OpenGL).");
                });
            });
            egui::Window::new("egui ❤ macroquad").show(ctx, |ui| {
                let response =
                    ui.add(egui::Slider::new(pixels_per_point.as_mut().unwrap(), 0.75..=3.0).logarithmic(true));

                // Scale egui components based on the new value
                if response.drag_released() {
                    ctx.set_pixels_per_point(pixels_per_point.unwrap());
                }
            });
        });
        egui_macroquad::draw();

        // Draw macroquad ui components
        button.ui(&mut *root_ui(), screen(), None);
        if button.activated() {
            draw_rectangle(200., 300., 50., 50., RED);
        }

        next_frame().await
    }
}
