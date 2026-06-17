//! Slim sticky navigation bar that stays pinned to the top while scrolling, with jump
//! links to the page's three main sections.

use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "sticky top-0 z-50 bg-biltong-900/95 backdrop-blur text-biltong-50 shadow-sm",
            div { class: "max-w-5xl mx-auto px-4 sm:px-6 h-14 flex items-center justify-between gap-3",
                a {
                    href: "#top",
                    class: "font-display font-bold text-lg whitespace-nowrap hover:text-white transition-colors",
                    "Biltong"
                }
                div { class: "flex items-center gap-1 sm:gap-2 text-sm font-semibold",
                    NavLink { href: "#calculator", label: "Calculator" }
                    NavLink { href: "#cuts", label: "Cuts" }
                    NavLink { href: "#steps", label: "Steps" }
                }
            }
        }
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> Element {
    rsx! {
        a {
            href,
            class: "px-2.5 py-1.5 rounded-md text-biltong-100 hover:bg-biltong-700 hover:text-white transition-colors",
            "{label}"
        }
    }
}
