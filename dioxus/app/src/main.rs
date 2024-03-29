#[cfg(any(windows, unix))]
use rux::components::TitleBar;
use rux::{
    components::Section,
    elements::{Appearance, ArrowPosition, Button, Switch, Tooltip},
    icons::HiOutlineIcon,
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
                // Allows rounded CSS window effect to work
                .with_transparent(true)
                // Turns off standard window manager controls
                .with_decorations(false)
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

    // Desktop window titlebar and controls
    let mut titlebar: Option<VNode> = None;
    #[cfg(any(windows, unix))]
    {
        titlebar = cx.render(rsx! {
            TitleBar{
                 text: "Pre-release | Issues/Feedback".into(),
                 link: "https://issues.satellite.im".into()
            }
        })
    }

    // Get the themes
    let theme = rux::get_available_themes().iter().find(|x| x.name == "Nord").unwrap().styles.clone();

    cx.render(rsx! {
        style { "{STYLES} {theme}" },
        div {
            id: "root",
            titlebar,
            Section {
                section_label: "Checkbox - unchecked".into(),
                section_description: "Example of a checkbox that is unchecked".into(),
                Switch {
                    active: false,
                },
            },
            div {
                style: "padding-left: 10px; border-bottom: 1px solid var(--border-color); color: var(--text-color);
    ",
                p {
                    "Button Examples",
                },
                Button {
                    text: "Default".into(),
                    appearance: Appearance::Default,
                    icon: HiOutlineIcon::User,
                },
                                Button {
                    text: "Primary".into(),
                    appearance: Appearance::Primary,
                    icon: HiOutlineIcon::Cog6Tooth,
                },
                Button {
                    text: "Secondary".into(),
                    appearance: Appearance::Secondary,
                    icon: HiOutlineIcon::MusicalNote,
                },
                Button {
                    text: "SecondaryLess".into(),
                    appearance: Appearance::SecondaryLess,
                    icon: HiOutlineIcon::LockClosed,
                },
                Button {
                    text: "Success".into(),
                    appearance: Appearance::Success,
                    icon: HiOutlineIcon::Folder,
                },
                Button {
                    text: "Danger".into(),
                    appearance: Appearance::Danger,
                    icon: HiOutlineIcon::Beaker,
                },
                Button {
                    text: "Disabled".into(),
                    appearance: Appearance::Disabled,
                    icon: HiOutlineIcon::BellAlert,
                },
                Button {
                    text: "Transparent".into(),
                    appearance: Appearance::Transparent,
                    icon: HiOutlineIcon::EyeSlash,
                }
                Button {
                    text: "With Badge".into(),
                    with_badge: "Badge".into(),
                    appearance: Appearance::Default,
                    icon: HiOutlineIcon::CommandLine,
                },
            },
            Section {
                section_label: "Small Button".into(),
                section_description: "Setting small=true provides room only for the icon".into(),
                Button {
                    small: true,
                    icon: HiOutlineIcon::Activity,
                },
            },
            Section {
                section_label: "Custom Tooltip".into(),
                section_description: "Creating a custom tool tip for the button".into(),
                Button {
                    text: "Custom Tooltip".into(),
                    icon: HiOutlineIcon::Calendar,
                    appearance: Appearance::Primary,
                    tooltip: cx.render(rsx!(
                        Tooltip {
                            arrow_position: ArrowPosition::Bottom,
                            text: String::from("Settings")
                        }
                    )),
                },
            }
        }
    })
}
