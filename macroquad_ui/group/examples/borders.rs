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

fn font_color(color: Color) -> Skin {
    let label_style =
        root_ui().style_builder().text_color(color).text_color_hovered(color).text_color_clicked(color).build();
    Skin { label_style, ..root_ui().default_skin() }
}

#[macroquad::main(main_conf)]
async fn main() {
    let columns = 4;
    let spacing = 10.0;
    let w = screen_width() / columns as f32 - spacing - (spacing / columns as f32);
    let grouper = GroupBuilder::new().size(Size::Static(w, w));
    let bg = Texture2D::from_file_with_format(include_bytes!("../assets/background.png"), None);
    let white_font = font_color(WHITE);
    let mut i = 0;
    let mut j = 0;

    // Create groups of various border types
    let mut pos = Position::Static(j as f32 * (w + spacing) + spacing, i as f32 * (w + spacing) + spacing);
    let mut bg_color_only = grouper.build("bg_color_only").with_position(pos).with_background_color(BLUE);
    //let mut bg_grp = grouper.build("bg_image_only").with_position(pos).with_background(bg.clone());

    let mut fps = Fps::new().with_position(Position::LeftBottom(rect(10., 0., 0., 0.)));
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        bg_color_only.ui(&mut *root_ui(), Size::screen(), |ui, size, offset| {
            ui.push_skin(&white_font);
            widgets::Label::new("bg color only").ui(ui);
            ui.pop_skin();
        });

        next_frame().await
    }
}
