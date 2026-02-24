use crate::Route;
use dioxus::prelude::*;
const CHEMICAL_SVG: Asset = asset!("/assets/chemical.svg");

#[component]
pub fn Navbar() -> Element {
    let mut mobile_open = use_signal(|| false);
    rsx! {
        nav { class: "relative bg-gray-800",

            div { class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
                div { class: "relative flex h-16 items-center justify-between",

                    // Mobile menu button
                    div { class: "absolute inset-y-0 left-0 flex items-center sm:hidden",
                        button {
                            class: "relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-white/5 hover:text-white",
                            onclick: move |_| mobile_open.toggle(),

                            span { class: "sr-only", "Open main menu" }

                            svg {
                                class: if !mobile_open() { "size-6" } else { "hidden" },
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                path {
                                    d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                }
                            }

                            svg {
                                class: if mobile_open() { "size-6" } else { "hidden" },
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                path {
                                    d: "M6 18 18 6M6 6l12 12",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                }
                            }
                        }
                    }

                    // Logo + desktop nav
                    div { class: "flex flex-1 items-center justify-center sm:items-stretch sm:justify-start",
                        div { class: "flex shrink-0 items-center",
                            img {
                                class: "h-8 w-auto",
                                src: CHEMICAL_SVG,
                                alt: "Logo",
                            }
                        }

                        div { class: "hidden sm:ml-6 sm:block",
                            div { class: "flex space-x-4",

                                Link {
                                    to: Route::Home {},
                                    class: "rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white",
                                    "Dashboard"
                                }

                                Link {
                                    to: Route::Sir {},
                                    class: "rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-white/5 hover:text-white",
                                    "SIR-model"
                                }

                                Link {
                                    to: Route::Regulation {},
                                    class: "rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-white/5 hover:text-white",
                                    "Regulation"
                                }
                            }
                        }
                    }
                }
            }

            // Mobile menu
            if mobile_open() {
                div { class: "sm:hidden space-y-1 px-2 pt-2 pb-3",

                    Link {
                        to: Route::Home {},
                        class: "block rounded-md bg-gray-900 px-3 py-2 text-base font-medium text-white",
                        "Dashboard"
                    }

                    Link {
                        to: Route::Sir {},
                        class: "block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-white/5 hover:text-white",
                        "SIR-model"
                    }

                    Link {
                        to: Route::Regulation {},
                        class: "block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-white/5 hover:text-white",
                        "Regulation"
                    }
                }
            }
        }
        div { class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8 mt-10", Outlet::<Route> {} }
    }
}
