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

        let mut base_layout = Layout::base().vert().fill_w().spacing(10.).margin(10., 10., 60., 10.);

        Region::new(RED).show(&mut *root_ui(), &mut base_layout);
        Region::new(BLUE).show(&mut *root_ui(), &mut base_layout);
        Region::new(GREEN).show(&mut *root_ui(), &mut base_layout);
        Region::new(ORANGE).show(&mut *root_ui(), &mut base_layout);
        Region::new(YELLOW).show(&mut *root_ui(), &mut base_layout);
        Region::new(BROWN).show(&mut *root_ui(), &mut base_layout);
        Region::new(BEIGE).show(&mut *root_ui(), &mut base_layout);
        Region::new(PURPLE).show(&mut *root_ui(), &mut base_layout);
        Region::new(PINK).show(&mut *root_ui(), &mut base_layout);

        next_frame().await
    }
}
