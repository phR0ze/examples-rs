//! Demonstrating visually layout::tests::layout_vertical
//! * Shows spacing, padding, margins affects
//! * Child layouts also have marings taking affect
//! * Third child layout shows expansion accounts for largest shape
//! * Third child layout has overflow correction of -10 due to constrained parent size
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "vertical".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let builder = PanelBuilder::new()
        .layout(|x| x.with_size_static(100., 100.).with_margins_all(10.))
        .frame(|x| x.with_fill(GRAY));

    loop {
        clear_background(WHITE);

        let mut p1 = Panel::new("0")
            .with_layout(|x| {
                x.with_size_static(210., 410.)
                    .with_mode(Mode::TopToBottom)
                    .with_spacing(10.)
                    .with_padding_all(20.)
                    .with_margins_all(10.)
            })
            .with_frame(|x| x.with_fill(BLACK));

        let mut c1 = builder.build("1").with_layout(|x| x.with_parent(&p1.layout()));
        let mut c2 = builder.build("2").with_layout(|x| x.with_parent(&p1.layout()));
        let mut c3 = builder.build("3").with_layout(|x| x.with_size_static(150., 100.).with_parent(&p1.layout()));

        p1.show(&mut *root_ui());
        c1.show(&mut *root_ui());
        c2.show(&mut *root_ui());
        c3.show(&mut *root_ui());

        next_frame().await
    }
}
