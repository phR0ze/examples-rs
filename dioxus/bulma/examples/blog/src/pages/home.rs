use bulma::prelude::*;

#[allow(non_snake_case)]
pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "tile is-ancestor is-vertical",
            div { class: "tile is-child hero",
                div { class: "hero-body container pb-5",
                    h1 { class: "title is-1",
                        "Welcome..."
                    },
                    h2 { class: "subtitle",
                        "...to the best yew content"
                    }
                },
                div { class: "tile is-child",
                    figure { class: "image is-3by1",
                        img { src: "https://source.unsplash.com/random/1200x400/?yew" }
                    }
                },
                div { class: "tile is-parent container",
                    div { class: "tile is-parent",
                        div { class: "tile is-child box",
                            p { class: "title",
                                "What are yews?"
                            }
                            p { class: "subtitle",
                                "Everything you need to know!"
                            }
                            div { class: "content",
                                r#"
                                A yew is a small to medium-sized evergreen tree, growing 10 to 20 metres tall, with a trunk up to 2 metres in diameter.
                                The bark is thin, scaly brown, coming off in small flakes aligned with the stem.
                                The leaves are flat, dark green, 1 to 4 centimetres long and 2 to 3 millimetres broad, arranged spirally on the stem,
                                but with the leaf bases twisted to align the leaves in two flat rows either side of the stem,
                                except on erect leading shoots where the spiral arrangement is more obvious.
                                The leaves are poisonous.
                                "#
                            }
                        }
                    }
                    div { class: "tile is-parent",
                        div { class: "tile is-child box",
                            p { class: "title",
                                "Who are we?"
                            }
                            div { class: "content",
                                "We're a small team of just 2"
                                sup {
                                    "64"
                                }
                                " members working tirelessly to bring you the low-effort yes conent we all desperately crave."
                                br {}
                                r#"
                                We put a ton of effort into fact-checking our posts.
                                Some say they read like a Wikipedia article - what a compliment!
                                "#
                            }
                        }
                    }
                }
            }
        }
    })
}
