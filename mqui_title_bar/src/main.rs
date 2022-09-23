use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};

mod group;
mod menu;
mod position;
mod titlebar;
use menu::{Menu, MenuEntry, MenuStyle};
use titlebar::{TitleBar, TitleBarStyle};

// Mobile device screens have the same or better pixel density as full monitors
// but are tiny, so its necessary to scale up the rendered results.
#[cfg(not(target_os = "android"))]
pub const SCALE_MULTIPLIER: f32 = 1.0;
#[cfg(target_os = "android")]
pub const SIZE_MULTIPLIER: f32 = 4.0;
pub fn scale(value: f32) -> f32 {
    value * SCALE_MULTIPLIER
}
pub fn scale_vec2(x: f32, y: f32) -> Vec2 {
    vec2(x * SCALE_MULTIPLIER, y * SCALE_MULTIPLIER)
}
pub fn scale_rect(left: f32, right: f32, top: f32, bottom: f32) -> RectOffset {
    RectOffset::new(
        left * SCALE_MULTIPLIER,
        right * SCALE_MULTIPLIER,
        top * SCALE_MULTIPLIER,
        bottom * SCALE_MULTIPLIER,
    )
}

pub struct Resources {
    menu_style: MenuStyle,
    title_bar_style: TitleBarStyle,
}
impl Resources {
    pub fn load() -> Self {
        // Load assets from app data
        let font_htowert = include_bytes!("../assets/HTOWERT.TTF");
        let menu_bg = Image::from_file_with_format(include_bytes!("../assets/menu_bg.png"), None);
        let menu_btn = Image::from_file_with_format(include_bytes!("../assets/menu_btn.png"), None);
        let menu_btn_clk = Image::from_file_with_format(include_bytes!("../assets/menu_btn_clk.png"), None);
        let options_btn = Image::from_file_with_format(include_bytes!("../assets/options_btn.png"), None);
        let options_btn_clk = Image::from_file_with_format(include_bytes!("../assets/options_btn_clk.png"), None);
        let entry_bg = Image::from_file_with_format(include_bytes!("../assets/entry_bg.png"), None);
        let entry_hov_bg = Image::from_file_with_format(include_bytes!("../assets/entry_hov_bg.png"), None);
        let entry_clk_bg = Image::from_file_with_format(include_bytes!("../assets/entry_clk_bg.png"), None);

        Resources {
            menu_style: MenuStyle {
                background: menu_bg,
                padding: scale_rect(20., 20., 20., 20.),
                spacing: scale(10.),
                entry_bg,
                entry_clk_bg,
                entry_hov_bg,
                entry_font: font_htowert,
                entry_font_size: scale(40.) as u16,
                entry_font_color: Color::from_rgba(180, 180, 100, 255),
                entry_padding: scale_rect(0.0, 0.0, 10.0, 10.0),
            },
            title_bar_style: TitleBarStyle {
                padding: scale_rect(15., 15., 5., 5.),
                title_font: font_htowert,
                title_font_size: scale(30.) as u16,
                title_font_color: Color::from_rgba(250, 250, 250, 250),
                menu_btn,
                menu_btn_clk,
                options_btn,
                options_btn_clk,
            },
        }
    }
}

fn main_conf() -> Conf {
    Conf {
        window_title: "mqui_title_bar".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    // Note: it is critical that resources and skins are loaded and configured
    // outside the main loop, else you'll get flickering and odd ui behavior.
    let resources = Resources::load();
    let mut titlebar = TitleBar::new(hash!("titlebar"), "Title Bar", resources.title_bar_style);
    let menu = Menu::new(
        hash!("menu"),
        scale_vec2(250., 250.),
        &[MenuEntry::new("Play1"), MenuEntry::new("Settings1"), MenuEntry::new("Quit1")],
        resources.menu_style.clone(),
    );
    let options = Menu::new(
        hash!("options"),
        scale_vec2(250., 250.),
        &[MenuEntry::new("Play2"), MenuEntry::new("Settings2"), MenuEntry::new("Quit2")],
        resources.menu_style,
    );

    loop {
        clear_background(BLACK);

        titlebar.ui(&mut *root_ui());
        if titlebar.menu() {
            menu.ui(&mut *root_ui());
        }
        if titlebar.options() {
            options.ui(&mut *root_ui());
        }

        next_frame().await
    }
}
