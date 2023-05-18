use crate::content::{self, Generated};
use bulma::{
    dioxus_router::{use_route, use_router},
    elements::*,
    prelude::*,
};

#[allow(non_snake_case)]
pub fn Post(cx: Scope) -> Element {
    let route = use_route(cx);
    let content = route.parse_segment::<u64>("post").unwrap_or(default);

    let content = match route.parse_segment::<u64>("post") {
        Some(result) => match result {
            Ok(seed) => {
                // Generate the same content based on the identified seed
                let author = content::Author::generate_from_seed(seed);

                rsx! {
                    div { class: "section container",
                        div { class: "tile is-ancestor is-vertical",
                            div { class: "tile is-parent",
                                article { class: "tile is-child notification is-light",
                                    p { class: "title",
                                        author.name
                                    }
                                }
                            }
                            div { class: "tile",
                                div { class: "tile is-parent is-3",
                                    article { class: "tile is-child notification",
                                        p { class: "title",
                                            "Interests"
                                        }
                                        div { class: "tags",
                                            for tag in author.keywords {
                                                span { class: "tag is-info",
                                                    tag
                                                }
                                            }
                                        }
                                    }
                                }
                                div { class: "tile is-parent",
                                    figure { class: "tile is-child image is-square",
                                        img { src: "{author.image_url}" }
                                    }
                                }
                                div { class: "tile is-parent",
                                    article { class: "tile is-child notification is-info",
                                        div { class: "content",
                                            Title { "About me" }
                                            div { class: "content",
                                                "This author has chosen not to reveal anything about themselves"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => rsx! {
                use_router(cx).replace_route("", None, None)
            },
        },
        _ => rsx! {
            use_router(cx).replace_route("", None, None)
        },
    };
    cx.render(rsx! {
        content
    })
}
