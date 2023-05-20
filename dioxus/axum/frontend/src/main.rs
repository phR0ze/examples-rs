use dioxus::prelude::*;
use dioxus_logger;
use fermi::*;

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    #[cfg(target_family = "wasm")]
    dioxus_web::launch(App);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(dioxus_desktop::LogicalSize::new(1200, 700)),
        ),
    )
}

// UI entry point that will only get called once on startup
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    fermi::use_init_atom_root(&cx);

    cx.render(rsx! {
        //style { "{get_bulma_css()}" },
        // Router {
        //     pages::Header {},
        //     pages::Footer {}
        // }
        NotFoundPage {}
    })
}

#[allow(non_snake_case)]
fn NotFoundPage(cx: Scope) -> Element {
    cx.render(rsx! { p { "404 Page not found - Dioxus" } })
}
