//! Interactive spice calculator: enter the meat weight, pick a unit system, and every
//! ingredient is scaled and re-rendered reactively.

use crate::calculator::{compute, meat_to_grams, UnitSystem};
use dioxus::prelude::*;

#[component]
pub fn Calculator() -> Element {
    let mut system = use_signal(|| UnitSystem::Metric);
    // Keep the raw text (so the field shows exactly what was typed) and the parsed value.
    let mut raw = use_signal(|| "1".to_string());

    let sys = system();
    let amount = raw().trim().parse::<f64>().unwrap_or(0.0).max(0.0);
    let lines = compute(meat_to_grams(amount, sys), sys);

    rsx! {
        section { id: "calculator", class: "bg-biltong-50 py-16",
            div { class: "max-w-2xl mx-auto px-6",
                h2 { class: "font-display text-3xl font-bold text-biltong-700 text-center mb-3",
                    "Recipe Calculator"
                }
                p { class: "text-center text-stone-600 mb-8",
                    "Enter how much beef you're starting with and the spice cure scales to match."
                }

                div { class: "bg-white rounded-2xl shadow-sm ring-1 ring-biltong-100 p-6 sm:p-8",
                    // Controls
                    div { class: "flex flex-col sm:flex-row gap-4 sm:items-end mb-6",
                        label { class: "flex-1",
                            span { class: "block text-sm font-semibold text-stone-700 mb-1",
                                "Amount of beef"
                            }
                            div { class: "flex",
                                input {
                                    r#type: "number",
                                    min: "0",
                                    step: "0.1",
                                    inputmode: "decimal",
                                    value: "{raw}",
                                    class: "w-full rounded-l-lg border border-biltong-300 px-3 py-2 \
                                            focus:outline-none focus:ring-2 focus:ring-biltong-500",
                                    oninput: move |evt| raw.set(evt.value()),
                                }
                                span { class: "inline-flex items-center px-4 rounded-r-lg border \
                                               border-l-0 border-biltong-300 bg-biltong-50 \
                                               text-stone-600 font-medium",
                                    "{sys.meat_unit()}"
                                }
                            }
                        }
                        // Unit toggle
                        div { class: "inline-flex rounded-lg ring-1 ring-biltong-300 overflow-hidden",
                            UnitButton { label: "Metric", active: sys == UnitSystem::Metric,
                                onclick: move |_| system.set(UnitSystem::Metric) }
                            UnitButton { label: "Imperial", active: sys == UnitSystem::Imperial,
                                onclick: move |_| system.set(UnitSystem::Imperial) }
                        }
                    }

                    // Results
                    if amount > 0.0 {
                        table { class: "w-full text-left",
                            tbody {
                                for line in lines {
                                    tr { key: "{line.name}", class: "border-t border-biltong-100",
                                        td { class: "py-2.5 pr-4",
                                            span { class: "font-medium text-stone-800", "{line.name}" }
                                            if !line.note.is_empty() {
                                                span { class: "block text-xs text-stone-400",
                                                    "{line.note}"
                                                }
                                            }
                                        }
                                        td { class: "py-2.5 text-right font-semibold text-biltong-700 \
                                                     whitespace-nowrap",
                                            "{line.amount}"
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        p { class: "text-center text-stone-400 py-6",
                            "Enter an amount of beef to see the spice cure."
                        }
                    }
                }
                p { class: "text-xs text-stone-400 text-center mt-4",
                    "Ratios are a starting point — adjust salt and spice to your own taste."
                }
            }
        }
    }
}

#[component]
fn UnitButton(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let classes = if active {
        "bg-biltong-700 text-biltong-50"
    } else {
        "bg-white text-stone-600 hover:bg-biltong-50"
    };
    rsx! {
        button {
            r#type: "button",
            class: "px-4 py-2 text-sm font-semibold transition-colors {classes}",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
