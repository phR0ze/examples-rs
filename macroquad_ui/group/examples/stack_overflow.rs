use core::prelude::*;
use macroquad::ui::Drag;

fn main_conf() -> Conf {
    Conf {
        window_title: "test".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

pub fn test<F: FnOnce(&mut Ui)>(ui: &mut Ui, size: Vec2, id: &str, f: F) -> Drag {
    widgets::Group::new(hash!(id), size).ui(ui, f)
}

#[macroquad::main(main_conf)]
async fn main() {
    loop {
        clear_background(WHITE);

        let size1 = vec2(300., 300.);
        let size2 = vec2(250., 250.);

        test(&mut *root_ui(), size1, "foo1", |ui| {
            test(ui, size2, "foo2", |ui2| {});
        });
        // widgets::Group::new(hash!(), size1).ui(&mut *root_ui(), |ui| {
        //     widgets::Group::new(hash!(), size2).ui(ui, |ui| {});
        // });

        next_frame().await
    }
}
