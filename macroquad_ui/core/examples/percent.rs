use core::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "core".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

fn skin_with_color(color: Color) -> Skin {
    let button_style = root_ui().style_builder().color(color).color_hovered(color).color_clicked(color).build();
    Skin { button_style, ..root_ui().default_skin() }
}

#[macroquad::main(main_conf)]
async fn main() {
    let gray_skin = skin_with_color(GRAY);

    loop {
        clear_background(WHITE);
        root_ui().push_skin(&gray_skin);

        // 1st button using individual width and height functions
        widgets::Button::new("Width: 45%, Height: 25%")
            .size(vec2(
                Width::Percent(0.45).relative(Size::screen()),
                Height::Percent(0.25).relative(Size::screen()),
            ))
            .position(vec2(10., 10.))
            .ui(&mut *root_ui());

        // 2nd button using size functions
        widgets::Button::new("Size: (45%, 25%)")
            .size(Size::Percent(0.45, 0.25).relative(Size::screen()))
            .position(vec2(230., 10.))
            .ui(&mut *root_ui());

        // 3rd button using size functions
        widgets::Button::new("Size: (95%, 50%)")
            .size(Size::Percent(0.95, 0.50).relative(Size::screen()))
            .position(vec2(10., 250.))
            .ui(&mut *root_ui());

        root_ui().pop_skin();
        next_frame().await
    }
}
