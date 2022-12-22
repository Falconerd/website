use dioxus::{
    prelude::*,
    router::{Route, Router, Link}
};

mod poisson_disk_sampling;

#[allow(non_snake_case)]
fn PostList(cx: Scope) -> Element {
    let post_links: Vec<(&str, &str)> = vec![("Poisson Disk Sampling and its Uses in Games", "/poisson-disk-sampling")];

    cx.render(rsx!{
        ol {
            post_links.iter().map(|link| rsx!(li { Link { to: "{link.1}", "{link.0}" } }))
        }
    })
}

#[allow(non_snake_case)]
fn HomePage(cx: Scope) -> Element {
    cx.render(rsx!{
        p {
            "Hello there! My name is Dylan Falconer and this is my website."
        }
        p {
            "Here you will find interactive articles about programming and game development."
        }
        PostList {}
        hr {}
        p {
            "Both the front-end and back-end are written in Rust. The front-end is powered by Dioxus, a library similar to React. The back-end is powered by Rocket, a library similar to Express."
        }
    })
}

#[allow(non_snake_case)]
fn ErrorPage(cx: Scope) -> Element {
    cx.render(rsx! { "not found" })
}

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            div {
                max_width: "640px",
                margin_left: "auto",
                margin_right: "auto",
                header {
                    h1 { Link { to: "/", "Dylan Falconer's Website" } }
                }
                main {
                    hr {}
                    Route { to: "/", self::HomePage {}}
                    Route { to: "/poisson-disk-sampling", poisson_disk_sampling::render {}}
                    Route { to: "", self::ErrorPage {}}
                    hr {}
                }
                footer {
                    p { "If you would like to help support my work, that would be greatly appreciated." }
                    p { "You can get access to more source code including the source code for my commercial game projects via my " a { href: "#", "Subscribe Star" } "." }
                    p { "Otherwise, crypto donations are very welcome :)" }
                    pre {
                        "Bitcoin: bc1xxxxx57y\nMonero:  48jxxxxx9bh"
                    }
                }
            }
        }
    })
}
