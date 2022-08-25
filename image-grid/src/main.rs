use macroquad::prelude::*;

#[macroquad::main("Image Grid")]
async fn main() {
    // Initialization
    let columns = 4;
    let thickness = 2.;
    let (w, h) = (screen_width(), screen_height());
    let sq_item_size = w / columns as f32;

    loop {
        // Controls
        if is_key_pressed(KeyCode::Space) {
            //animation.toggle_paused();
        }

        // Presentation
        clear_background(BLACK);

        // Draw grid
        for i in 0..columns + 1 {
            // Draw row
            let (rx, ry) = (0., (i * sq_item_size as i32) as f32);
            draw_line(rx, ry, rx + w, ry, thickness, WHITE);

            // Draw column
            let (cx, cy) = ((i * sq_item_size as i32) as f32, 0.);
            draw_line(cx, cy, cx, cy + h, thickness, WHITE);
        }

        next_frame().await
    }
}
