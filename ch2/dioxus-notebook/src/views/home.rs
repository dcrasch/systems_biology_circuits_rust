use dioxus::prelude::*;
use crate::components::*;
use comrak::{Options, markdown_to_html};

const MARKDOWN_CSS: Asset = asset!("/assets/markdown.css");
static MARKDOWN_SOURCE: &str = r#"
## Welcome

* [Dioxus](https://dioxuslabs.com/learn/0.7/)
* [Tailwind](https://tailwindcss.com/docs/)
"#;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MARKDOWN_CSS }
        div {
            class: "markdown-body",
            dangerous_inner_html: markdown_to_html(MARKDOWN_SOURCE, &Options::default()),
        }
    }
}
