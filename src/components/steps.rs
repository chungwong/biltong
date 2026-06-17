//! The tutorial steps. Each step is a card with its number, copy and an animated SVG,
//! alternating the illustration left/right on wider screens.

use crate::animations::StepArt;
use crate::calculator::{drying_temp, CalcInput};
use crate::recipe::{Step, STEPS};
use dioxus::prelude::*;

#[component]
pub fn Steps() -> Element {
    rsx! {
        section { id: "steps", class: "max-w-4xl mx-auto px-6 py-16",
            h2 { class: "font-display text-3xl font-bold text-biltong-700 text-center mb-12",
                "The Steps"
            }
            div { class: "flex flex-col gap-10",
                for (index , step) in STEPS.iter().enumerate() {
                    StepCard { key: "{index}", index, step }
                }
            }
        }
    }
}

#[component]
fn StepCard(index: usize, step: &'static Step) -> Element {
    // Alternate the illustration side on sm+ screens.
    let media_order = if index % 2 == 0 {
        "sm:order-1"
    } else {
        "sm:order-2"
    };
    let text_order = if index % 2 == 0 {
        "sm:order-2"
    } else {
        "sm:order-1"
    };

    // Substitute the unit-aware drying temperature into the body (no-op for other steps).
    let system = use_context::<Signal<CalcInput>>()().system;
    let body = step.body.replace("{temp}", drying_temp(system));

    rsx! {
        article { class: "grid sm:grid-cols-2 gap-6 items-center bg-white rounded-2xl \
                          shadow-sm ring-1 ring-biltong-100 p-6",
            div { class: "bg-biltong-50 rounded-xl p-2 {media_order}",
                StepArt { kind: step.anim }
            }
            div { class: "{text_order}",
                div { class: "flex items-center gap-3 mb-2",
                    span { class: "flex items-center justify-center w-9 h-9 rounded-full \
                                   bg-biltong-700 text-biltong-50 font-bold",
                        "{index + 1}"
                    }
                    h3 { class: "font-display text-2xl font-semibold text-biltong-900",
                        "{step.title}"
                    }
                }
                p { class: "text-stone-600 leading-relaxed", "{body}" }
            }
        }
    }
}
