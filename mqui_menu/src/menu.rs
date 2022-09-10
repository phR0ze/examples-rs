//! Menu encapsulates and automates the manipulation of a set of widgets to provide
//! typical menu type functionality.
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets, Id, Skin, Ui},
};

use crate::{group::Group, position::Position, style::MenuStyle};

#[derive(Debug, Default, Clone)]
pub struct MenuEntry {
    pub title: String,
}

pub struct Menu {
    id: Id,
    skin: Skin,
    style: MenuStyle,
    group: Group,
    entries: Vec<MenuEntry>,
}

impl Menu {
    /// Create a new instance
    pub fn new(id: Id, size: Vec2, entries: &[MenuEntry], style: MenuStyle) -> Self {
        // Group handles making the menu relative to the app window during resizes
        let group_bg = style.background.clone();
        let group = Group::new(id, size, group_bg).padding(style.padding);

        // Configure menu and button styles
        let skin = Skin { button_style: style.entry(), ..root_ui().default_skin() };
        Menu { id, skin, style, group, entries: entries.to_vec() }
    }

    /// Center the menu on the screen
    #[allow(dead_code)]
    pub fn center(&mut self) -> &Self {
        self.group.position(Position::Center);
        self
    }

    /// Position the menu on the screen
    #[allow(dead_code)]
    pub fn position(&mut self, pos: Vec2) -> &Self {
        self.group.position(pos);
        self
    }

    /// Draw the menu on the screen
    pub fn ui(&self, ui: &mut Ui) {
        self.group.ui(ui, |ui, size| {
            ui.push_skin(&self.skin);

            // Draw the regular menu entries
            for (i, entry) in self.entries.iter().enumerate() {
                let size = self.style.entry_size(size);
                let pos = self.style.entry_pos(i);
                widgets::Button::new(entry.title.as_str()).size(size).position(pos).ui(ui);
            }

            ui.pop_skin();
        });
    }
}
