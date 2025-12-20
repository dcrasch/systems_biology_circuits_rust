use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Regulation() -> Element {
    rsx! {
        LineChart1 {}
	LineChart2 {}
    }
}
