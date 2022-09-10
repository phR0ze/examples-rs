use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin, Ui},
};

#[macroquad::main("margins")]
async fn main() {
    let margin = {
        let button_style = root_ui()
            .style_builder()
            .color(BLUE)
            .color_hovered(BLUE)
            .margin(RectOffset::new(20.0, 20.0, 20.0, 20.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();
        Skin { button_style, ..root_ui().default_skin() }
    };
    let bg_margin = {
        let button_style = root_ui()
            .style_builder()
            .color(BLUE)
            .color_hovered(BLUE)
            .background_margin(RectOffset::new(20.0, 20.0, 20.0, 20.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();
        Skin { button_style, ..root_ui().default_skin() }
    };

    let btn_bg = Image::from_file_with_format(include_bytes!("../assets/btn_bg.png"), None);
    let btn_hov_bg = Image::from_file_with_format(include_bytes!("../assets/btn_hov_bg.png"), None);
    let btn_clk_bg = Image::from_file_with_format(include_bytes!("../assets/btn_clk_bg.png"), None);

    let bg = {
        let button_style = root_ui()
            .style_builder()
            .background(btn_bg.clone())
            .background_clicked(btn_clk_bg.clone())
            .background_hovered(btn_hov_bg.clone())
            .margin(RectOffset::new(20.0, 20.0, 20.0, 20.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();
        Skin { button_style, ..root_ui().default_skin() }
    };

    let bg_bg_margin = {
        let button_style = root_ui()
            .style_builder()
            .background(btn_bg)
            .background_clicked(btn_clk_bg)
            .background_hovered(btn_hov_bg)
            .background_margin(RectOffset::new(20.0, 20.0, 20.0, 20.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();
        Skin { button_style, ..root_ui().default_skin() }
    };

    loop {
        clear_background(GRAY);
        let mut x = 20.0;
        let mut y = 20.0;
        let w = 200.0;
        let h = 60.0;
        let font_size = 30.0;

        // Draw title
        draw_text("Buttons w/1px border to visually show margin impact.", x, y, font_size, BLACK);
        y += font_size;

        // Draw text explanations
        draw_text("margin(20.,20.,20.,20.)", x, y + 20., font_size - 10., BLACK);
        draw_text("background_margin(20...", x + 250., y + 20., font_size - 10., BLACK);
        draw_text("size(200.0, 60.0)", x + 500., y + 20., font_size - 10., BLACK);
        y += font_size;

        // color bg - margin
        draw_rectangle(x - 1.0, y - 1.0, w + 2., h + 2.0, WHITE);
        root_ui().push_skin(&margin);
        widgets::Button::new("__ margin").position(vec2(x, y)).ui(&mut *root_ui());
        root_ui().pop_skin();

        // color bg - background_margin
        x += 250.0;
        draw_rectangle(x - 1.0, y - 1.0, w + 2., h + 2.0, WHITE);
        root_ui().push_skin(&bg_margin);
        widgets::Button::new("bg margin").position(vec2(x, y)).ui(&mut *root_ui());
        root_ui().pop_skin();

        // size so no margin at all
        x += 250.0;
        draw_rectangle(x - 1.0, y - 1.0, w + 2., h + 2.0, WHITE);
        root_ui().push_skin(&bg_margin);
        widgets::Button::new("size()").size(vec2(w, h)).position(vec2(x, y)).ui(&mut *root_ui());
        root_ui().pop_skin();

        // Draw text explanations
        x -= 500.0;
        y += h + 40.0;
        draw_text("margin(20.,20.,20.,20.)", x, y + 20., font_size - 10., BLACK);
        draw_text("background_margin(20...", x + 250., y + 20., font_size - 10., BLACK);
        draw_text("size(200.0, 60.0)", x + 500., y + 20., font_size - 10., BLACK);
        y += font_size;

        // bg - margin
        draw_rectangle(x - 1.0, y - 1.0, w + 2., h + 2.0, WHITE);
        root_ui().push_skin(&bg);
        widgets::Button::new("__ margin").position(vec2(x, y)).ui(&mut *root_ui());
        root_ui().pop_skin();

        // bg - background_margin
        x += 250.0;
        draw_rectangle(x - 1.0, y - 1.0, w + 2., h + 2.0, WHITE);
        root_ui().push_skin(&bg_bg_margin);
        widgets::Button::new("bg margin").position(vec2(x, y)).ui(&mut *root_ui());
        root_ui().pop_skin();

        // bg - size so no margin at all
        x += 250.0;
        draw_rectangle(x - 1.0, y - 1.0, w + 2., h + 2.0, WHITE);
        root_ui().push_skin(&bg_bg_margin);
        widgets::Button::new("size()").size(vec2(w, h)).position(vec2(x, y)).ui(&mut *root_ui());
        root_ui().pop_skin();

        x -= 500.0;
        y += h + 60.0;
        draw_text("margin() seems to distort the background image while", x, y, font_size, BLACK);
        y += font_size;
        draw_text("background_margin() simply stretches it to fit", x, y, font_size, BLACK);
        y += font_size;
        draw_text("the additional space the margin added.", x, y, font_size, BLACK);

        // Group
        widgets::Group::new(hash!(), group_size).position(vec2(x, y)).ui(&mut *root_ui(), |ui| {
            //ui.pop_skin();
        });

        next_frame().await;
    }
}
