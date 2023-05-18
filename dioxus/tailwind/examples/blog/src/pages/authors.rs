use crate::{
    content::{self, Generated},
    ROUTES,
};
use bulma::{components::*, dioxus_router::Link, elements::*, layouts::*, prelude::*};
use rand::{distributions::Standard, Rng};

static SIGNAL: AtomRef<bool> = |_| false;
static PROGRESS: AtomRef<Progress> = |_| Progress::default();

#[allow(non_snake_case)]
pub fn Authors(cx: Scope) -> Element {
    println!("render authors page");
    let signal = use_atom_ref(cx, SIGNAL);

    // Generate authors
    let seeds: Vec<u64> = rand::thread_rng().sample_iter(Standard).take(2).collect();
    let authors: Vec<content::Author> =
        seeds.iter().map(|&seed| content::Author::generate_from_seed(seed)).collect();
    let id = format!("{}/{}:{}", ROUTES.authors, seeds[0], seeds[1]);

    // Create progress state
    cx.render(rsx! {
        Container {
            section { class: "hero",
                div { class: "hero-body",
                    Container {
                        Title { "Authors" }
                        SubTitle { "Meet the definitely real peaple behind your favorite Yew content" }
                    }
                }
            }
            p { class: "section py-0",
                "It wouldn't be fair"
                i { "(or possible :P)" }
                " to list each and every author in alphabetical order."
                br{}
                "So instead we chose to put more focus on the individuals by introducing you to two people at a time"
            }
            div { class: "section",
                div { class: "tile is-ancestor",
                    for author in (authors) {
                        div { class: "tile is-parent",
                            div { class: "tile is-child",
                                AuthorCard { name: author.name,
                                    seed: author.seed,
                                    keywords: author.keywords,
                                    img_src: author.image_url,
                                }
                            }
                        }
                    }
                }
                ProgressTimed { id: id,
                    state: PROGRESS,
                    color: Colors::Primary,
                    completed: signal,
                }
            }
        }
    })
}

#[allow(non_snake_case)]
#[derive(PartialEq, Props)]
pub struct AuthorProps {
    #[props(!optional)]
    seed: u64,

    #[props(!optional)]
    name: String,

    #[props(!optional)]
    keywords: Vec<String>,

    #[props(!optional)]
    img_src: String,
}

#[allow(non_snake_case)]
pub fn AuthorCard<'a>(cx: Scope<'a, AuthorProps>) -> Element {
    let seed = cx.props.seed;

    cx.render(rsx! {
        Card {
            CardContent {
                Media {
                    MediaLeft {
                        Image { size: 128,
                            src: &cx.props.img_src,
                        }
                    }
                    MediaContent {
                        Title { size: 3, cx.props.name.clone() }
                        p { "I like "
                            b { cx.props.keywords.join(", ") }
                        }
                    }
                }
            }
            CardFooter {
                Link { class: "card-footer-item"
                    to: "/authors/{seed}", "Profile",
                }
            }
        }
    })
}
