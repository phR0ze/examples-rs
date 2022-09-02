use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Skin, Ui},
};

mod panel;
use panel::Panel;

pub struct Resources {
    font_htowert: &'static [u8],
    image_win_bg: Image,
    image_btn_bg: Image,
    image_btn_hov_bg: Image,
    image_btn_clk_bg: Image,
    skin_main_menu: Skin,
}
impl Resources {
    pub fn load() -> Self {
        // Load assets from static memory
        let font_htowert = include_bytes!("../assets/HTOWERT.TTF");
        let image_win_bg =
            Image::from_file_with_format(include_bytes!("../assets/win_bg.png"), None);
        let image_btn_bg =
            Image::from_file_with_format(include_bytes!("../assets/btn_bg.png"), None);
        let image_btn_hov_bg =
            Image::from_file_with_format(include_bytes!("../assets/btn_hov_bg.png"), None);
        let image_btn_clk_bg =
            Image::from_file_with_format(include_bytes!("../assets/btn_clk_bg.png"), None);

        // Configure main menu skin
        let skin_main_menu = {
            let label_style = root_ui()
                .style_builder()
                .font(font_htowert)
                .unwrap()
                .text_color(Color::from_rgba(180, 180, 120, 255))
                .font_size(30)
                .build();

            let window_style = root_ui()
                .style_builder()
                .background(image_win_bg.clone())
                .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
                .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(image_btn_bg.clone())
                .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
                .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
                .background_hovered(image_btn_hov_bg.clone())
                .background_clicked(image_btn_clk_bg.clone())
                .font(font_htowert)
                .unwrap()
                .text_color(Color::from_rgba(180, 180, 100, 255))
                .font_size(40)
                .build();

            let editbox_style = root_ui()
                .style_builder()
                .background_margin(RectOffset::new(0., 0., 0., 0.))
                .font(font_htowert)
                .unwrap()
                .text_color(Color::from_rgba(120, 120, 120, 255))
                .color_selected(Color::from_rgba(190, 190, 190, 255))
                .font_size(50)
                .build();

            Skin {
                editbox_style,
                window_style,
                button_style,
                label_style,
                ..root_ui().default_skin()
            }
        };

        Resources {
            font_htowert,
            image_win_bg,
            image_btn_bg,
            image_btn_hov_bg,
            image_btn_clk_bg,
            skin_main_menu,
        }
    }
}

#[macroquad::main("main menu")]
async fn main() {
    // Note: it is critical that resources and skins are loaded and configured
    // outside the main loop, else you'll get flickering and odd ui behavior.
    let resources = Resources::load();
    let menu_size = vec2(300., 300.);
    let mut menu = Panel::new(
        &mut *root_ui(),
        hash!(),
        menu_size,
        resources.image_win_bg.clone(),
    );

    loop {
        clear_background(WHITE);

        menu.center().ui(&mut *root_ui(), |ui, inner_size| {
            ui.push_skin(&resources.skin_main_menu);
            widgets::Button::new("Play")
                .position(vec2(65.0, 15.0))
                .ui(ui);
            widgets::Button::new("Options")
                .position(vec2(40.0, 75.0))
                .ui(ui);

            widgets::Button::new("Quit")
                .position(vec2(65.0, 195.0))
                .ui(ui);
            ui.pop_skin();
        });

        next_frame().await
    }
}
