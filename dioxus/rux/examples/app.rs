use rux::{
    components::Section,
    elements::{Icon, Switch},
    icons::fa_brands_icons::FaRust,
    prelude::*,
    STYLES,
};

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(App);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_title("rux")
                .with_resizable(true)
                // Provides rounded window corner effect
                .with_transparent(true)
                // Turns off standard window manager controls
                //.with_decorations(false)
                // We start the min inner size smaller because the prelude pages like unlock can be rendered much smaller.
                .with_min_inner_size(dioxus_desktop::LogicalSize::new(300.0, 350.0))
                .with_inner_size(dioxus_desktop::LogicalSize::new(950.0, 600.0)),
        ),
    )
}

// UI entry point
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    //use_shared_state_provider(cx, || State::default());
    #[cfg(any(windows, unix))]
    println!("CWD: {:?}", std::env::current_dir());

    let theme = rux::get_available_themes().iter().find(|x| x.name == "Nord").unwrap().styles.clone();

    cx.render(rsx! {
        style { "{STYLES} {theme}" },
        div {
            // Titlebar {
            //     text: "Pre-release | Issues/Feedback".into(),
            //     link: "https://issues.satellite.im".into()
            // },
            // Routes{},
            Icon {
                width: 30,
                height: 30,
                fill: "black",
                icon: FaRust,
            },
            Section {
                section_label: "Checkbox - unchecked".into(),
                section_description: "Example of a checkbox that is unchecked".into(),
                Switch {
                    active: false,
                },
            },
            p {
                "Hello world!"
            }
        }
    })
}
