use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        LineChart {}
        LineChart2 {}
        LineChart3 {}
    }
}
