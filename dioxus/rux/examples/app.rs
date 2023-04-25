use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_title("rux")
                .with_resizable(true)
                // Provides rounded window corner effect
                //.with_transparent(true)
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
    //let theme = get_available_themes().iter().find(|x| x.name == "Nord").unwrap().styles.clone();

    cx.render(rsx! {
        //style { "{STYLE} {APP_STYLE} {theme}" },
        div {
            // Titlebar {
            //     text: "Pre-release | Issues/Feedback".into(),
            //     link: "https://issues.satellite.im".into()
            // },
            // Routes{},
            "Hello world!"
        }
    })
}
