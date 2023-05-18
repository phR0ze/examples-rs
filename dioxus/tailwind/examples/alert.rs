use dioxus::prelude::*;
use tailwind::{components::*, prelude::*};

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string()),
    )
}

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Alert {
            "Change a few things up and try submitting again."
        }
    })
}
