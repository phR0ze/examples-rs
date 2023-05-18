use crate::{
    content::{self, Generated},
    ROUTES,
};
use bulma::{components::*, elements::*, layouts::*, prelude::*};
use rand::Rng;

static PAGINATION: AtomRef<Pagination> = |_| Pagination::default();

#[allow(non_snake_case)]
pub fn Posts(cx: Scope) -> Element {
    let pagination = use_atom_ref(&cx, PAGINATION);

    let cols = 3;
    let per_page = 9;
    let per_col = per_page / cols;

    // Generate content
    use_future(cx, (), |_| {
        to_owned![pagination];
        async move {
            // Generate some random total page count
            let pages = rand::thread_rng().gen_range(10..100);
            pagination.write().set(ROUTES.posts, (1..=pages).map(|x| x.to_string()).collect::<Vec<String>>());
        }
    });

    // Generate content for the indicated page
    let start_seed = pagination.read().current_page(ROUTES.posts) * per_page;
    let mut posts =
        (0..per_page).map(|seed_offset| content::PostMeta::generate_from_seed((start_seed + seed_offset) as u64));

    cx.render(rsx! {
        Section {
            Container {
                is_fluid: true,
                Title { "Posts" }
                SubTitle { "All of our quality writing in one place!" }
                Columns {
                    for _ in (1..=cols) {
                        Column {
                            List {
                                for post in posts.by_ref().take(per_col) {
                                    PostCard { title: post.title,
                                        author: post.author.name,
                                        img_src: post.image_url,
                                    }
                                }
                            }
                        }
                    }
                }
                Pagination { url: ROUTES.posts.into(),
                    state: pagination,
                }
            }
        }
    })
}

#[allow(non_snake_case)]
#[derive(PartialEq, Props)]
pub struct PostProps {
    #[props(!optional)]
    title: String,

    #[props(!optional)]
    author: String,

    #[props(!optional)]
    img_src: String,
}

#[allow(non_snake_case)]
pub fn PostCard(cx: Scope<PostProps>) -> Element {
    cx.render(rsx! {
        ListItem { class: "mb-5".into(),
            Card {
                CardImage {
                    Image { ratio: (2, 1).into(),
                        src: &cx.props.img_src,
                    }
                }
                CardContent {
                    Title { cx.props.title.clone() }
                    SubTitle { cx.props.author.clone() }
                    // span { class: "icon-text",
                    //     span { class: "is-uppercase has-text-weight-medium is-size-7",
                    //         "Read More"
                    //     }
                    //     span { class: "icon",
                    //         Icon {
                    //             width: 15,
                    //             height: 15,
                    //             icon: fa_solid_icons::FaArrowRight,
                    //         }
                    //     }
                    // }
                }
            }
        }
    })
}
