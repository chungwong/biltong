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
    // Largest amount drives the full-width bar; everything else scales relative to it.
    let max_mag = lines
        .iter()
        .map(|l| l.magnitude)
        .fold(0.0_f64, f64::max)
        .max(1e-9);

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

                    // Results — a bar per ingredient, sized by amount, labelled with the
                    // live quantity. Recomputes whenever the weight or unit changes.
                    if amount > 0.0 {
                        svg {
                            class: "w-full",
                            view_box: "0 0 300 150",
                            role: "img",
                            "aria-label": "Ingredient amounts",
                            for (i , l) in lines.iter().enumerate() {
                                g { key: "{l.short}",
                                    text {
                                        x: "80",
                                        y: "{20.0 + i as f64 * 23.0}",
                                        text_anchor: "end",
                                        font_size: "9",
                                        fill: "#44403c",
                                        "{l.short}"
                                    }
                                    rect {
                                        x: "86",
                                        y: "{11.0 + i as f64 * 23.0}",
                                        width: "150",
                                        height: "13",
                                        rx: "3",
                                        fill: "#f1e7dd",
                                    }
                                    rect {
                                        x: "86",
                                        y: "{11.0 + i as f64 * 23.0}",
                                        width: "{150.0 * l.magnitude / max_mag}",
                                        height: "13",
                                        rx: "3",
                                        fill: "{l.color}",
                                    }
                                    text {
                                        x: "242",
                                        y: "{21.0 + i as f64 * 23.0}",
                                        font_size: "9",
                                        font_weight: "bold",
                                        fill: "#7c2d12",
                                        "{l.amount}"
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
