//! Demonstrating panel basics
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "panel".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    loop {
        clear_background(WHITE);

        Panel::new("p1")
            .with_layout(|x| {
                x.with_mode(Mode::TopToBottom).with_spacing(10.).with_size_full().with_padding_all(30.)
            })
            .with_frame(|x| x.with_fill(BLACK))
            .show_f(&mut *root_ui(), |ui, layout| {
                Panel::new("p2")
                    .with_layout(|x| {
                        x.with_mode(Mode::LeftToRight)
                            .with_size_static(390., 200.0)
                            .with_spacing(10.)
                            .with_padding_all(20.)
                    })
                    .with_frame(|x| x.with_fill(DARKGRAY))
                    .show_pf(ui, Some(layout), |ui, layout| {
                        Panel::new("0")
                            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.))
                            .with_frame(|x| x.with_fill(RED))
                            .show_p(ui, layout);
                        Panel::new("1")
                            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.))
                            .with_frame(|x| x.with_fill(GRAY))
                            .show_p(ui, layout);
                        Panel::new("2")
                            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.))
                            .with_frame(|x| x.with_fill(BLUE))
                            .show_p(ui, layout);
                    });
                Panel::new("p3")
                    .with_layout(|x| {
                        x.with_mode(Mode::LeftToRight)
                            .with_size_static(390., 200.0)
                            .with_spacing(10.)
                            .with_padding_all(20.)
                    })
                    .with_frame(|x| x.with_fill(GREEN))
                    .show_pf(ui, Some(layout), |ui, layout| {
                        Panel::new("0")
                            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.))
                            .with_frame(|x| x.with_fill(RED))
                            .show_p(ui, layout);
                        Panel::new("1")
                            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.))
                            .with_frame(|x| x.with_fill(GRAY))
                            .show_p(ui, layout);
                        Panel::new("2")
                            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.))
                            .with_frame(|x| x.with_fill(BLUE))
                            .show_p(ui, layout);
                    });
            });

        next_frame().await
    }
}
