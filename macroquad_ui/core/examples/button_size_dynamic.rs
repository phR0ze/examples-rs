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

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::new().align(Align::LeftBottom(rect(10., 0., 0., 10.)));

    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        let mut btn1 = Button1::new("Dynamic Sizing")
            .with_background_color(BLUE)
            .with_position(Align::LeftTop(rect(10., 0., 10., 0.)));
        btn1.ui(&mut *root_ui(), screen(), None);
        let size1 = btn1.size(&mut *root_ui(), screen());

        Button1::new("Dynamic Sizing w/Padding")
            .with_background_color(BLUE)
            .with_padding(20., 20., 20., 20.)
            .with_position(Align::LeftTop(rect(10., 0., 20. + size1.y, 0.)))
            .ui(&mut *root_ui(), screen(), None);

        Button1::new("3/4 W, Dynamic H")
            .with_background_color(BLUE)
            .with_size(Size::three_quarter_width())
            .with_position(Align::LeftTop(None))
            .ui(&mut *root_ui(), screen(), offset(0., 150.));

        Button1::new("Dynamic W, 100px H")
            .with_background_color(BLUE)
            .with_padding(0., 10., 0., 0.)
            .with_size(Size::CalcWidth(Width::Dynamic, 100.))
            .with_position(Align::LeftTop(None))
            .ui(&mut *root_ui(), screen(), offset(0., 200.));

        next_frame().await
    }
}
