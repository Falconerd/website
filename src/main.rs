use dioxus::{
    prelude::*,
    router::{Route, Router, Link}
};
use markdown::Markdown;

mod markdown;
mod poisson_disk_sampling;

#[allow(non_snake_case)]
fn HomePage(cx: Scope) -> Element {
    //let mut context: CanvasRenderingContext2d = CanvasRenderingContext2d { obj: js_sys::Object::new() };

    cx.render(rsx!{
        Markdown { content: include_str!("homepage/intro.md") }
        hr {}
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
                    id: "main-header",
                    Link { to: "/", "Dylan Falconer's Website" }
                }
                hr {}
                main {
                    Route { to: "/", self::HomePage {}}
                    Route { to: "/poisson-disk-sampling", poisson_disk_sampling::render {}}
                    Route { to: "/blog", self::HomePage {} }
                    Route { to: "/blog/:post", self::BlogPost {} }
                    //Route { to: "/devlogs/2023-02-01", self::DevlogPage{} }
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

#[allow(non_snake_case)]
fn BlogPost(cx: Scope) -> Element {
    let route = use_route(&cx);
    let id = route.segment("post").unwrap_or("");

    cx.render(rsx! {
        Markdown { content: match id {
            "dawnbreaker-devlog-1" => include_str!("devlogs/01.md"),
            _ => "Blog post does not exist"
    }}})
}
