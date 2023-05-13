use crate::{
    content::{self, Generated},
    ROUTES,
};
use bulma::{components::*, dioxus_router::Link, elements::*, fermi::Readable, layouts::*, prelude::*};
use rand::{distributions, Rng};

#[allow(non_snake_case)]
pub fn Authors(cx: Scope) -> Element {
    println!("render authors page");

    // Generate authors
    let seeds: Vec<u64> = rand::thread_rng().sample_iter(distributions::Standard).take(2).collect();
    let authors: Vec<content::Author> =
        seeds.iter().map(|&seed| content::Author::generate_from_seed(seed)).collect();
    let id = format!("{}/{}:{}", ROUTES.authors, seeds[0], seeds[1]);

    // Create progress state
    let progress = ProgressState::new_with_notify(cx);

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
                RefreshAuthors {
                    id: id,
                    state: progress,
                }
            }
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props, PartialEq)]
pub struct RefreshAuthorsProps {
    id: String,
    state: fermi::AtomRef<ProgressState>,
}

/// By pushing the timed progress bar into a sub-component we can keep the parent component
/// page from re-rendering over and over. Instead only this component is re-rendered each
/// time the timer fires.
#[allow(non_snake_case)]
pub fn RefreshAuthors(cx: Scope<RefreshAuthorsProps>) -> Element {
    let state = fermi::use_atom_ref(cx, cx.props.state);
    cx.render(rsx! {
        ProgressTimed { id: cx.props.id.clone(),
            state: state,
            color: Colors::Primary,
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
