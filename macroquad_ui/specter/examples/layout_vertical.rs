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
    let builder = PanelBuilder::new().layout(|x| x.size_s(100., 100.).margins_all(10.)).frame(|x| x.fill(GRAY));

    loop {
        clear_background(WHITE);

        let mut p1 = Panel::vert("0")
            .layout(|x| x.size_s(210., 410.).spacing(10.).padding_all(20.).margins_all(10.))
            .frame(|x| x.fill(BLACK));

        p1.append(builder.build("1"));
        p1.append(builder.build("2"));
        p1.append(builder.build("3"));
        // let mut c1 = builder.build("1").layout(|x| x.parent(&p1));
        // let mut c2 = builder.build("2").layout(|x| x.parent(&p1));
        // let mut c3 = builder.build("3").layout(|x| x.size_s(150., 100.).parent(&p1));

        p1.show(&mut *root_ui());
        // c1.show(&mut *root_ui());
        // c2.show(&mut *root_ui());
        // c3.show(&mut *root_ui());

        next_frame().await
    }
}
