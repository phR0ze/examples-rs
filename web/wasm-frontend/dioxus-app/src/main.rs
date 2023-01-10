use dioxus::prelude::*;
use gloo::console::log;

fn main() {
    log!("Root log: ", "web app booting up");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "hello, wasm!" }
    })
}
