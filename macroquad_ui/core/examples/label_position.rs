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
    loop {
        clear_background(WHITE);
        let spacing = 10.;

        // Row 1
        let cont_size = vec2(100., 50.);
        let pos1 = vec2(10., 10.);
        widgets::Group::new(hash!(), cont_size).position(pos1).ui(&mut *root_ui(), |ui| {
            Label::new("center top ").with_position(Align::CenterTop(None)).ui(ui, cont_size, None);
        });
        let pos2 = vec2(pos1.x + cont_size.x + spacing, pos1.y);
        widgets::Group::new(hash!(), cont_size).position(pos2).ui(&mut *root_ui(), |ui| {
            Label::new("center").with_position(Align::Center(None)).ui(ui, cont_size, None);
        });
        let pos3 = vec2(pos2.x + cont_size.x + spacing, pos2.y);
        widgets::Group::new(hash!(), cont_size).position(pos3).ui(&mut *root_ui(), |ui| {
            Label::new("center bottom").with_position(Align::CenterBottom(None)).ui(ui, cont_size, None);
        });
        let pos4 = vec2(pos3.x + cont_size.x + spacing, pos3.y);
        widgets::Group::new(hash!(), cont_size).position(pos4).ui(&mut *root_ui(), |ui| {
            Label::new("left top").with_position(Align::LeftTop(None)).ui(ui, cont_size, None);
        });

        // Row 2
        let pos5 = vec2(pos1.x, pos1.y + cont_size.y + spacing);
        widgets::Group::new(hash!(), cont_size).position(pos5).ui(&mut *root_ui(), |ui| {
            Label::new("left center").with_position(Align::LeftCenter(None)).ui(ui, cont_size, None);
        });
        let pos6 = vec2(pos1.x + cont_size.x + spacing, pos1.y + cont_size.y + spacing);
        widgets::Group::new(hash!(), cont_size).position(pos6).ui(&mut *root_ui(), |ui| {
            Label::new("left bottom").with_position(Align::LeftBottom(None)).ui(ui, cont_size, None);
        });
        let pos7 = vec2(pos6.x + cont_size.x + spacing, pos6.y);
        widgets::Group::new(hash!(), cont_size).position(pos7).ui(&mut *root_ui(), |ui| {
            Label::new("right top").with_position(Align::RightTop(None)).ui(ui, cont_size, None);
        });
        let pos8 = vec2(pos7.x + cont_size.x + spacing, pos7.y);
        widgets::Group::new(hash!(), cont_size).position(pos8).ui(&mut *root_ui(), |ui| {
            Label::new("right center").with_position(Align::RightCenter(None)).ui(ui, cont_size, None);
        });

        // Row 3
        let pos9 = vec2(pos1.x, pos6.y + cont_size.y + spacing);
        widgets::Group::new(hash!(), cont_size).position(pos9).ui(&mut *root_ui(), |ui| {
            Label::new("right bottom").with_position(Align::RightBottom(None)).ui(ui, cont_size, None);
        });
        let pos10 = vec2(pos9.x + cont_size.x + spacing, pos9.y);
        widgets::Group::new(hash!(), cont_size).position(pos10).ui(&mut *root_ui(), |ui| {
            Label::new("blue font").with_font_color(BLUE).ui(ui, cont_size, None);
        });
        let pos11 = vec2(pos10.x + cont_size.x + spacing, pos10.y);
        widgets::Group::new(hash!(), cont_size).position(pos11).ui(&mut *root_ui(), |ui| {
            Label::new("red hover").with_font_color_hov(RED).ui(ui, cont_size, None);
        });

        next_frame().await
    }
}
