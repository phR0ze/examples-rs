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
    let gray_bldr = ButtonBuilder1::new().background_color(GRAY).label_font_size(15.);
    let mut fps = Fps::new().align(Align::LeftBottom(rect(10., 0., 0., 10.)));

    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        root_ui().push_skin(&gray_skin);

        // 1st button using individual width and height functions
        gray_bldr
            .build("Width: 45%, Height: 25%")
            .with_size(Size::Calc(Width::Percent(0.45), Height::Percent(0.25)))
            .with_position(Align::LeftTop(None))
            .ui(&mut *root_ui(), screen(), offset(10., 10.));

        // 2nd button using size functions
        gray_bldr
            .build("Size: (45%, 25%)")
            .with_size(Size::Percent(0.45, 0.25))
            .with_position(Align::Static(230., 10.))
            .ui(&mut *root_ui(), screen(), None);

        // 3rd button using size functions
        gray_bldr
            .build("Size: (95%, 50%)")
            .with_size(Size::Percent(0.95, 0.50))
            .with_position(Align::Static(10., 250.))
            .ui(&mut *root_ui(), screen(), None);

        root_ui().pop_skin();
        next_frame().await
    }
}
