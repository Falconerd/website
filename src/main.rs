use dioxus::{
    prelude::*,
    router::{Route, Router, Link}
};

mod markdown;
mod poisson_disk_sampling;

use markdown::Markdown;

#[allow(non_snake_case)]
fn HomePage(cx: Scope) -> Element {
    cx.render(rsx!{
        Markdown { content: include_str!("homepage/intro.md") }
        Markdown { content: include_str!("post_list.md") }
        hr {}
        Markdown { content: include_str!("homepage/outro.md") }
    })
}

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            div {
                id: "container",
                header {
                    h1 { Link { to: "/", "Dylan Falconer's Website" } }
                }
                hr {}
                main {
                    Route { to: "/", self::HomePage {}}
                    Route { to: "/poisson-disk-sampling", poisson_disk_sampling::render {}}
                    Route { to: "", "Page not found." }
                }
                footer {
                    // hr {}
                    // Markdown { content: include_str!("footer.md") }
                }
            }
        }
    })
}

