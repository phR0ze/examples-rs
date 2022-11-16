//! Demonstrating visually layout::tests::layout_horizontal
//! * Shows spacing, padding, margins affects
//! * Child layouts also have marings taking affect
//! * Third child layout shows expansion accounts for largest shape
//! * Third child layout has overflow correction of -10 due to constrained parent size
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "horizontal".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::new().layout(|x| x.align(Align::LeftBottom).margins_all(5.));
    let builder = PanelBuilder::new().layout(|x| x.size_s(100., 100.).margins_all(10.)).frame(|x| x.fill(GRAY));

    loop {
        clear_background(WHITE);

        Panel::horz(id!())
            .layout(|x| x.size_s(410., 210.).spacing(10.).padding_all(20.).margins_all(10.))
            .frame(|x| x.fill(BLACK))
            .add(builder.build(id!()))
            .add(builder.build(id!()))
            .add(builder.build(id!()).layout(|x| x.size_s(100., 150.)))
            .show(&mut *root_ui());

        fps.show(&mut *root_ui(), None);

        next_frame().await
    }
}
