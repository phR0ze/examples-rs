//! Demonstrating visually layout::tests::layout_fill_height
//! * Shows parent precentage size
//! * Shows margins and spacing
//! * Third item shows difference in width
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "fill height".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let builder = PanelBuilder::new().layout(|x| x.size_s(100., 100.)).frame(|x| x.fill(GRAY));

    loop {
        clear_background(WHITE);

        Panel::horz(id!())
            .layout(|x| x.size_f().spacing(10.).margins_all(10.).fill_h())
            .frame(|x| x.fill(BLACK))
            .add(builder.build(id!()))
            .add(builder.build(id!()))
            .add(builder.build(id!()).layout(|x| x.size_s(150., 100.)))
            .show();

        next_frame().await
    }
}
