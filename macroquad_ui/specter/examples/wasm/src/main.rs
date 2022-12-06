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

        draw_text("what up dog?", 40., 40., 32., ORANGE);
        draw_rectangle(100., 100., 100., 100., GREEN);
        draw_circle(150., 150., 10., RED);
        draw_line(100., 250., 200., 100., 2., BLUE);

        next_frame().await
    }
}
