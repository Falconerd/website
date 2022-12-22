// Copied from DioxusLabs/markdown.

use::dioxus::prelude::*;
use pulldown_cmark::Parser;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct MarkdownProps<'a> {
    content: &'a str,
}

#[allow(non_snake_case)]
pub fn Markdown<'a>(cx: Scope<'a, MarkdownProps<'a>>) -> Element {
    let parser = Parser::new(cx.props.content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    cx.render(rsx! {
        div {
            dangerous_inner_html: "{html_buf}"
        }
    })
}

