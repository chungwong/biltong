//! Top-of-page hero: title, intro and jump links to the two main sections.

use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        header { class: "bg-biltong-700 text-biltong-50",
            div { class: "max-w-3xl mx-auto px-6 py-16 sm:py-20 text-center",
                p { class: "uppercase tracking-widest text-sm text-biltong-300 mb-3",
                    "South African dried beef"
                }
                h1 { class: "font-display text-4xl sm:text-5xl font-bold mb-5",
                    "How to Make Biltong"
                }
                p { class: "text-lg text-biltong-100 leading-relaxed mb-8",
                    "A simple, step-by-step guide to making biltong at home — plus a "
                    "calculator that scales the spice cure to any amount of beef."
                }
                nav { class: "flex flex-wrap gap-3 justify-center",
                    a {
                        href: "#calculator",
                        class: "px-5 py-2.5 rounded-full bg-biltong-50 text-biltong-700 \
                                font-semibold hover:bg-white transition-colors",
                        "Recipe calculator"
                    }
                    a {
                        href: "#cuts",
                        class: "px-5 py-2.5 rounded-full border border-biltong-300 \
                                text-biltong-50 font-semibold hover:bg-biltong-900 transition-colors",
                        "Beef cuts"
                    }
                    a {
                        href: "#steps",
                        class: "px-5 py-2.5 rounded-full border border-biltong-300 \
                                text-biltong-50 font-semibold hover:bg-biltong-900 transition-colors",
                        "The steps"
                    }
                }
            }
        }
    }
}
