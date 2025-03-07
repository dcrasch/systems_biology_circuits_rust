use crate::components::LineChart;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        LineChart {}
    }
}
