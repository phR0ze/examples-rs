//! Demonstrating visually layout::tests::layout_fill_width
//! * Shows parent precentage size
//! * Shows margins and spacing
//! * Third item shows difference in height
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "fill width".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let builder = Panel::default().layout(|x| x.size_s(100., 100.)).frame(|x| x.fill(GRAY));

    loop {
        clear_background(WHITE);

        Panel::vert(id!())
            .layout(|x| x.size_p(0.75, 1.).spacing(10.).margins_all(10.).fill_w())
            .frame(|x| x.fill(BLACK))
            .add(builder.build(id!()))
            .add(builder.build(id!()))
            .add(builder.build(id!()).layout(|x| x.size_s(100., 150.)))
            .show();

        next_frame().await
    }
}
