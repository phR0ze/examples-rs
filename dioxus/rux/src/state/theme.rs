use serde::{Deserialize, Serialize};

#[derive(Eq, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Theme {
    pub filename: String,
    pub name: String,
    pub styles: String,
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
