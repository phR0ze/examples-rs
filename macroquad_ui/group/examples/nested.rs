use core::prelude::*;
use group::prelude::*;
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

pub struct TestGroup {
    size: Vec2,
    position: Vec2,
}

impl TestGroup {
    pub fn new(size: Vec2) -> TestGroup {
        TestGroup { size, position: vec2(0., 0.) }
    }

    pub fn position(self, position: Vec2) -> Self {
        TestGroup { position, ..self }
    }

    pub fn ui<F: FnOnce(&mut Ui)>(self, ui: &mut Ui, f: F) -> Drag {
        widgets::Group::new(hash!(), self.size).position(self.position).ui(ui, f)
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.ui(&mut *root_ui());

        let group1_size = vec2(300., 300.);
        let group1_pos = vec2(screen_width() - group1_size.x, screen_height() - group1_size.y) / 2.0;
        TestGroup::new(group1_size).position(group1_pos).ui(&mut *root_ui(), |ui| {
            widgets::Button::new("button 1").ui(ui);
            let group2_size = vec2(250., 250.);
            let group2_pos = vec2(group1_size.x - group2_size.x, group1_size.y - group2_size.y) / 2.0;
            TestGroup::new(group2_size).position(group2_pos).ui(ui, |ui2| {
                widgets::Button::new("button 1").ui(ui2);
            });
        });
        // let group1_size = vec2(300., 300.);
        // let group1_pos = vec2(screen_width() - group1_size.x, screen_height() - group1_size.y) / 2.0;
        // widgets::Group::new(hash!(), group1_size).position(group1_pos).ui(&mut *root_ui(), |ui| {
        //     widgets::Button::new("button 1").ui(ui);
        //     let group2_size = vec2(250., 250.);
        //     let group2_pos = vec2(group1_size.x - group2_size.x, group1_size.y - group2_size.y) / 2.0;
        //     widgets::Group::new(hash!(), group2_size).position(group2_pos).ui(ui, |ui| {
        //         widgets::Button::new("button 1").ui(ui);
        //         let group3_size = vec2(200., 200.);
        //         let group3_pos = vec2(group2_size.x - group3_size.x, group2_size.y - group3_size.y) / 2.0;
        //         widgets::Group::new(hash!(), group3_size).position(group3_pos).ui(ui, |ui| {
        //             widgets::Button::new("button 1").ui(ui);
        //             let group4_size = vec2(150., 150.);
        //             let group4_pos = vec2(group3_size.x - group4_size.x, group3_size.y - group4_size.y) / 2.0;
        //             widgets::Group::new(hash!(), group4_size).position(group4_pos).ui(ui, |ui| {
        //                 widgets::Button::new("button 1").ui(ui);
        //                 let group5_size = vec2(100., 100.);
        //                 let group5_pos = vec2(group4_size.x - group5_size.x, group4_size.y - group5_size.y) / 2.0;
        //                 widgets::Group::new(hash!(), group5_size).position(group5_pos).ui(ui, |ui| {
        //                     widgets::Button::new("button 1").ui(ui);
        //                 });
        //             });
        //         });
        //     });
        // });
        next_frame().await
    }
}
