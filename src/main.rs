use dioxus::prelude::*;

mod animations;
mod calculator;
mod components;
mod recipe;

use components::{Calculator, Footer, Hero, Steps};

/// Tailwind-compiled stylesheet (regenerated from `/tailwind.css` by the `dx` build).
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }

        div { class: "min-h-screen bg-biltong-50 text-stone-800",
            Hero {}
            main {
                Steps {}
                Calculator {}
            }
            Footer {}
        }
    }
}
