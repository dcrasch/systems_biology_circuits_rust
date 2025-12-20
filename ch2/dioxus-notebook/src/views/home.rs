use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
    div {
    class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
    	 h1 {
	 "Welcome"
	 }
	 ul {
	 class: "list-disc",
	 
         li {       a { href: "https://dioxuslabs.com/learn/0.7/", "Dioxus" }}
	 li {       a { href: "https://tailwindcss.com/docs/", "Tailwind" }}

            }	 
    }
    }
}
