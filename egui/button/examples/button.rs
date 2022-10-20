use egui::Widget;
pub use macroquad::{
    color::colors,
    prelude::*,
    ui::{hash, root_ui, widgets, Drag, Id, Skin, Style, Ui},
};

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
    let mut pixels_per_point: Option<f32> = None;

    loop {
        clear_background(BLACK);

        // Draw egui components
        egui_macroquad::ui(|ctx| {
            // Get current pixels per point
            if pixels_per_point.is_none() {
                //pixels_per_point = Some(ctx.pixels_per_point());
                pixels_per_point = Some(1.5);
                ctx.set_pixels_per_point(pixels_per_point.unwrap());
            }
            egui::CentralPanel::default().frame(egui::Frame::none().fill(egui::Color32::GRAY)).show(ctx, |ui| {
                let rect = egui::Rect::from_min_size(egui::pos2(50.0, 50.0), egui::vec2(50.0, 50.0));

                // Paint the frame:
                ui.painter().rect(rect, 0.0, egui::Color32::BLUE, egui::Stroke::none());

                // ui.vertical(|ui| {
                //     ui.spacing_mut().item_spacing.x = 0.0;
                //     let button = egui::Button::new("Click each year")
                //         .fill(egui::Color32::BLUE)
                //         .stroke(egui::Stroke::none());
                //     ui.put(rect, button);
                // });
            });
            // egui::Window::new("egui ❤ macroquad").show(ctx, |ui| {
            //     let response =
            //         ui.add(egui::Slider::new(pixels_per_point.as_mut().unwrap(), 0.75..=3.0).logarithmic(true));

            //     // Scale egui components based on the new value
            //     if response.drag_released() {
            //         ctx.set_pixels_per_point(pixels_per_point.unwrap());
            //     }
            // });
        });
        egui_macroquad::draw();

        next_frame().await
    }
}
