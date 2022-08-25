use macroquad::prelude::*;

#[macroquad::main("checker board")]
async fn main() {
    loop {
        for i in 0..=(screen_width() / 20.) as u32 {
            for j in 0..=(screen_height() / 20.) as u32 {
                draw_rectangle(
                    i as f32 * 20. - 10.,
                    j as f32 * 20. - 10.,
                    20.,
                    20.,
                    match (i + j) % 2 {
                        0 => Color::from_rgba(43, 46, 51, 255),
                        _ => Color::from_rgba(59, 62, 67, 255),
                    },
                )
            }
        }

        next_frame().await
    }
}
