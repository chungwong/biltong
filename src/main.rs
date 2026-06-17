use dioxus::prelude::*;

mod animations;
mod calculator;
mod components;
mod recipe;

use components::{Calculator, CutDiagram, Footer, Hero, NavBar, Steps};

/// Tailwind-compiled stylesheet (regenerated from `/tailwind.css` by the `dx` build).
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Shared calculator input, read by both the calculator and the step animations.
    use_context_provider(|| Signal::new(calculator::CalcInput::default()));

    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }

        div { class: "min-h-screen bg-biltong-50 text-stone-800",
            NavBar {}
            Hero {}
            main {
                Calculator {}
                CutDiagram {}
                Steps {}
            }
            Footer {}
        }
    }
}
