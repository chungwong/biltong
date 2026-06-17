//! Page footer with credit to the original tutorial.

use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-biltong-900 text-biltong-100",
            div { class: "max-w-3xl mx-auto px-6 py-10 text-center text-sm",
                p { class: "mb-2",
                    "Recipe and method adapted from the excellent biltong guide by "
                    a {
                        href: "https://twoguysandacooler.com/biltong/",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "underline hover:text-biltong-50",
                        "Two Guys & A Cooler"
                    }
                    "."
                }
                p { class: "text-biltong-300",
                    "Built with Dioxus + WebAssembly. No tracking, no server."
                }
            }
        }
    }
}
