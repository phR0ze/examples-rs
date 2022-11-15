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
    let builder = PanelBuilder::new().layout(|x| x.with_size_static(100., 100.)).frame(|x| x.with_fill(GRAY));

    loop {
        clear_background(WHITE);

        let mut p1 = Panel::vert("0")
            .with_layout(|x| {
                x.with_size_percent(0.75, 1.).with_spacing(10.).with_margins_all(10.).with_fill_width()
            })
            .with_frame(|x| x.with_fill(BLACK));

        let mut c1 = builder.build("1").with_layout(|x| x.with_parent(&p1.layout()));
        let mut c2 = builder.build("2").with_layout(|x| x.with_parent(&p1.layout()));
        let mut c3 = builder.build("3").with_layout(|x| x.with_size_static(100., 150.).with_parent(&p1.layout()));

        p1.show(&mut *root_ui());
        c1.show(&mut *root_ui());
        c2.show(&mut *root_ui());
        c3.show(&mut *root_ui());

        next_frame().await
    }
}
