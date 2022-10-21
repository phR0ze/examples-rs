use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "layout".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::new().with_font_color(WHITE);
    loop {
        clear_background(BLACK);
        fps.ui(&mut *root_ui());

        // container, defaults to full screen
        let cont1 = screen();
        draw_rectangle(0., 0., cont1.x, cont1.y, RED);

        Layout::new().add(vec2(50., 50.), BLUE).add(vec2(50., 50.), GREEN).show(&mut *root_ui(), |ui| {});

        next_frame().await
    }
}
