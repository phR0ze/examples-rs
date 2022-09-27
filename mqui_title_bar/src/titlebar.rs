//! TitleBar encapsulates and automates the manipulation of a set of widgets to provide
//! typical title bar type functionality.
use mqui_menu::prelude::*;

pub struct TitleBar {
    id: Id,       // title bar identifier
    group: Group, // provides layout for all titlebar widgets

    title: String,                     // title for the title bar
    title_font_size: u16,              // font size for the title
    title_font_color: Color,           // font color for the title
    title_position: Position,          // position for the title
    title_skin: Option<Skin>,          // title skin
    title_font: Option<&'static [u8]>, // font to use for the title

    menu: bool,                  // true if the menu button was pressed
    menu_enabled: bool,          // enable support for menu when true
    menu_skin: Option<Skin>,     // menu button skin
    menu_btn: Option<Image>,     // button image to use
    menu_btn_clk: Option<Image>, // button image to use when clicked

    options: bool,                  // true if the options button was pressed
    options_enabled: bool,          // enable support for options when true
    options_skin: Option<Skin>,     // options button skin
    options_btn: Option<Image>,     // button image to use
    options_btn_clk: Option<Image>, // button image to use when clicked
}

impl Default for TitleBar {
    fn default() -> Self {
        TitleBar {
            id: hash!(),
            group: Group::new(),
            title: String::default(),
            title_font: None,
            title_font_size: scale(30.0) as u16,
            title_font_color: Color::from_rgba(250, 250, 250, 250),
            title_position: Position::Center,
            title_skin: None,
            menu: false,
            menu_enabled: true,
            menu_skin: None,
            menu_btn: None,
            menu_btn_clk: None,
            options: false,
            options_enabled: true,
            options_skin: None,
            options_btn: None,
            options_btn_clk: None,
        }
    }
}

impl TitleBar {
    /// Create a new instance
    pub fn new<T: AsRef<str>>(title: T) -> Self {
        let title_bar = TitleBar::default();

        let title_font = include_bytes!("../assets/HTOWERT.TTF");
        let menu_btn = Image::from_file_with_format(include_bytes!("../assets/menu_btn.png"), None);
        let menu_btn_clk = Image::from_file_with_format(include_bytes!("../assets/menu_btn_clk.png"), None);
        let options_btn = Image::from_file_with_format(include_bytes!("../assets/options_btn.png"), None);
        let options_btn_clk = Image::from_file_with_format(include_bytes!("../assets/options_btn_clk.png"), None);

        // Calculate height of title bar
        let title_str = title.as_ref().to_string();
        let title_font_size = scale(30.0);
        let padding = RectOffset::new(15., 15., 5., 5.);
        root_ui().push_skin(&title_skin);
        let title_size = root_ui().calc_size(&title_str);
        let height = title_size.y + padding.top + padding.bottom;
        root_ui().pop_skin();

        let group = Group::new()
            .size(Size::FullWidth(height))
            .position(Position::TopCenter)
            .padding_p(padding)
            .border_color(BLUE);

        title_bar
    }

    /// Set the titlebar id
    pub fn id<T: Into<u64>>(self, id: T) -> Self {
        TitleBar { id: id.into(), ..self }
    }

    /// Set the titlebar's size
    /// * handles scaling for mobile
    pub fn size(self, size: Size) -> Self {
        TitleBar { group: self.group.size(size), ..self }
    }

    /// Position the titlebar on the screen
    pub fn position<T: Into<Position>>(self, pos: T) -> Self {
        TitleBar { group: self.group.position(pos), ..self }
    }

    /// Pad inside group pushing content in from edges
    /// * handles scaling for mobile
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        TitleBar { group: self.group.padding(left, right, top, bottom), ..self }
    }

    /// Returns true if the menu should be displayed
    pub fn menu(&self) -> bool {
        return self.menu;
    }

    /// Returns true if the options should be displayed
    pub fn options(&self) -> bool {
        return self.options;
    }

    /// Disable menu. This means it won't be displayed or taken into account
    pub fn disable_menu(self) -> Self {
        TitleBar { menu_enabled: false, ..self }
    }

    /// Disable options. This means it won't be displayed or taken into account
    pub fn disable_options(self) -> Self {
        TitleBar { options_enabled: false, ..self }
    }

    /// Position the title on the title bar
    pub fn title_position<T: Into<Position>>(self, pos: T) -> Self {
        TitleBar { title_position: pos.into(), ..self }
    }

    //  menu: false,
    //         menu_enabled: true,
    //         menu_skin: None,
    //         menu_btn: None,
    //         menu_btn_clk: None,
    //         options: false,
    //         options_enabled: true,
    //         options_skin: None,
    //         options_btn: None,
    //         options_btn_clk: None,

    /// Update the cached macroquad skin
    fn update_cached_skins(self) -> Self {
        let ui = root_ui();
        let title_style = ui
            .style_builder()
            .text_color(self.title_font_color)
            .text_color_hovered(self.title_font_color)
            .text_color_clicked(self.title_font_color)
            .font_size(self.title_font_size)
            .font(self.title_font)
            .unwrap()
            .build();

        let menu_style = ui
            .style_builder()
            .background(self.menu_btn.clone())
            .background_hovered(self.menu_btn.clone())
            .background_clicked(self.menu_btn_clk.clone())
            .build();

        let options_style = ui
            .style_builder()
            .background(self.options_btn.clone())
            .background_hovered(self.options_btn.clone())
            .background_clicked(self.options_btn_clk.clone())
            .build();

        TitleBar {
            title_skin: Skin { label_style: title_style, ..ui.default_skin() },
            menu_skin: Skin { button_style: menu_style, ..ui.default_skin() },
            options_skin: Skin { button_style: options_style, ..ui.default_skin() },
            ..self
        }
    }

    /// Draw the title bar and associated ui elements on the screen
    pub fn ui(&mut self, ui: &mut Ui) {
        // Draw the title bar
        self.group.ui(ui, |ui, size, pos| {
            // Draw title
            ui.push_skin(&self.title_skin);
            let title_size = ui.calc_size(&self.title);
            let title_position = self.title_position.relative(title_size, size, pos);
            ui.label(title_position, &self.title);
            ui.pop_skin();

            // Draw menu button and save state
            if self.menu_enabled {
                ui.push_skin(&self.menu_skin);
                let menu_size = vec2(title_size.y, title_size.y);
                let menu_pos = vec2(0.0, (size.y - menu_size.y) / 2.0);
                if widgets::Button::new("").size(menu_size).position(menu_pos).ui(ui) {
                    self.menu = !self.menu
                }
                ui.pop_skin();
            }

            // Draw options button and save state
            if self.options_enabled {
                ui.push_skin(&self.options_skin);
                let options_size = vec2(title_size.y, title_size.y);
                let options_pos = vec2(size.x - options_size.x, (size.y - options_size.y) / 2.0);
                if widgets::Button::new("").size(options_size).position(options_pos).ui(ui) {
                    self.options = !self.options
                }
                ui.pop_skin();
            }
        });
    }
}
