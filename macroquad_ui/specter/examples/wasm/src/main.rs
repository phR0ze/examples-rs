use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "wasm demo".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(WHITE);

        // Zoom for mobile?????
        // set_camera(&Camera2D {
        //     zoom: vec2(1., screen_width() / screen_height()),
        //     ..Default::default()
        // });

        draw_line(0., 0., 0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.1, 0.1, 0.2, 0.2, GREEN);
        draw_circle(0., 0., 0.1, RED);
        draw_text("what up dog?", 40., 40., 32., ORANGE);
        draw_rectangle(100., 100., 100., 100., GREEN);

        next_frame().await
    }
}
