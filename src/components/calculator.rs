//! Interactive spice calculator: enter the meat weight, pick a unit system, and every
//! ingredient is scaled and re-rendered reactively.

use crate::calculator::{compute, meat_to_grams, CalcInput, UnitSystem};
use dioxus::prelude::*;

#[component]
pub fn Calculator() -> Element {
    // Shared input so the step animations show the same live amounts.
    let mut input = use_context::<Signal<CalcInput>>();
    // Keep the raw text locally so the field shows exactly what was typed.
    let mut raw = use_signal(|| "1".to_string());

    let sys = input().system;
    let amount = parse_amount(&raw());
    let lines = compute(input().meat_grams, sys);

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
                    div { class: "mb-6 space-y-4",
                        // Unit toggle — on top, since it drives the unit shown everywhere below
                        div {
                            span { class: "block text-sm font-semibold text-stone-700 mb-1", "Units" }
                            div { class: "flex w-full sm:inline-flex sm:w-auto rounded-lg ring-1 ring-biltong-300 overflow-hidden",
                                UnitButton {
                                    label: "Metric",
                                    active: sys == UnitSystem::Metric,
                                    onclick: move |_| set_system(input, &raw(), UnitSystem::Metric),
                                }
                                UnitButton {
                                    label: "Imperial",
                                    active: sys == UnitSystem::Imperial,
                                    onclick: move |_| set_system(input, &raw(), UnitSystem::Imperial),
                                }
                            }
                        }
                        // Amount of beef
                        label { class: "block",
                            span { class: "block text-sm font-semibold text-stone-700 mb-1",
                                "Amount of beef"
                            }
                            div { class: "flex rounded-lg ring-1 ring-biltong-300 overflow-hidden",
                                button {
                                    r#type: "button",
                                    "aria-label": "Decrease amount",
                                    class: "px-4 bg-biltong-50 text-biltong-700 text-xl font-bold \
                                            leading-none hover:bg-biltong-100 transition-colors",
                                    onclick: move |_| step_amount(input, raw, -1.0),
                                    "−"
                                }
                                input {
                                    r#type: "number",
                                    min: "0",
                                    step: "0.1",
                                    inputmode: "decimal",
                                    value: "{raw}",
                                    class: "w-full min-w-0 border-0 px-3 py-2 text-center \
                                            focus:outline-none focus:ring-2 focus:ring-inset focus:ring-biltong-500",
                                    oninput: move |evt| {
                                        raw.set(evt.value());
                                        let grams = meat_to_grams(parse_amount(&evt.value()), input().system);
                                        input.write().meat_grams = grams;
                                    },
                                }
                                span { class: "inline-flex items-center px-3 bg-biltong-50 \
                                               text-stone-600 font-medium border-l border-biltong-200",
                                    "{sys.meat_unit()}"
                                }
                                button {
                                    r#type: "button",
                                    "aria-label": "Increase amount",
                                    class: "px-4 bg-biltong-50 text-biltong-700 text-xl font-bold \
                                            leading-none hover:bg-biltong-100 transition-colors \
                                            border-l border-biltong-200",
                                    onclick: move |_| step_amount(input, raw, 1.0),
                                    "+"
                                }
                            }
                        }
                        // Quick presets in the current unit
                        div {
                            span { class: "block text-sm font-semibold text-stone-700 mb-1", "Quick amounts" }
                            div { class: "flex flex-wrap gap-2",
                                for n in [1.0_f64, 2.0, 3.0, 4.0, 5.0] {
                                    button {
                                        key: "{n}",
                                        r#type: "button",
                                        class: if (amount - n).abs() < 1e-9 {
                                            "px-3 py-1.5 rounded-lg text-sm font-semibold border bg-biltong-700 text-white border-biltong-700"
                                        } else {
                                            "px-3 py-1.5 rounded-lg text-sm font-medium border border-biltong-300 text-stone-700 hover:bg-biltong-50 transition-colors"
                                        },
                                        onclick: move |_| set_amount(input, raw, n),
                                        "{n} {sys.meat_unit()}"
                                    }
                                }
                            }
                        }
                    }

                    // Results — exact amount per ingredient, recomputed whenever the
                    // weight or unit changes.
                    if amount > 0.0 {
                        ul { class: "divide-y divide-biltong-100",
                            for line in lines {
                                li { key: "{line.name}", class: "flex items-baseline justify-between gap-4 py-2.5",
                                    span { class: "min-w-0",
                                        span { class: "font-medium text-stone-800", "{line.name}" }
                                        if !line.note.is_empty() {
                                            span { class: "block text-xs text-stone-400", "{line.note}" }
                                        }
                                    }
                                    span { class: "font-semibold text-biltong-700 whitespace-nowrap",
                                        "{line.amount}"
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

/// Parse the weight field, clamping junk/negatives to 0.
fn parse_amount(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(0.0).max(0.0)
}

/// Step the meat amount via the −/+ buttons. The step is unit-aware (0.5 kg / 1 lb) since
/// native number spinners don't show on mobile and step too finely on desktop.
fn step_amount(mut input: Signal<CalcInput>, mut raw: Signal<String>, dir: f64) {
    let sys = input().system;
    let mag = match sys {
        UnitSystem::Metric => 0.5,
        UnitSystem::Imperial => 1.0,
    };
    let next = (parse_amount(&raw()) + dir * mag).max(0.0);
    // Whole numbers render without a trailing ".0".
    let text = if (next.fract()).abs() < 1e-9 {
        format!("{}", next.round() as i64)
    } else {
        format!("{next:.1}")
    };
    raw.set(text);
    input.write().meat_grams = meat_to_grams(next, sys);
}

/// Set the meat amount directly (used by the preset chips).
fn set_amount(mut input: Signal<CalcInput>, mut raw: Signal<String>, value: f64) {
    let sys = input().system;
    raw.set(format!("{value}"));
    input.write().meat_grams = meat_to_grams(value, sys);
}

/// Switch unit system, re-deriving the stored grams from the current raw input.
fn set_system(mut input: Signal<CalcInput>, raw: &str, system: UnitSystem) {
    let mut st = input.write();
    st.system = system;
    st.meat_grams = meat_to_grams(parse_amount(raw), system);
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
            class: "flex-1 sm:flex-none px-4 py-2 text-sm font-semibold transition-colors {classes}",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
