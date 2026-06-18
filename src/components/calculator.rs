//! Interactive spice calculator: enter the meat weight, pick a unit system, and every
//! ingredient is scaled and re-rendered reactively.

use crate::calculator::{
    compute, factor_from_amount, meat_to_grams, CalcInput, Overrides, UnitSystem,
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Calculator() -> Element {
    // Shared input so the step animations show the same live amounts.
    let mut input = use_context::<Signal<CalcInput>>();
    // Keep the raw text locally so the field shows exactly what was typed.
    let mut raw = use_signal(|| "1".to_string());
    // Per-ingredient amount overrides (as scaling factors), and the live text of whichever
    // ingredient field is being edited (so the field doesn't fight the recomputed value).
    let mut overrides = use_signal(Overrides::new);
    let mut edits = use_signal(HashMap::<String, String>::new);

    // Restore the unit, amount and ingredient overrides persisted from a previous visit.
    // Reading happens once on load, before any user action writes, so there's no clobber.
    use_future(move || async move {
        let mut eval = document::eval(
            "dioxus.send([localStorage.getItem('biltong:unit') || '', \
             localStorage.getItem('biltong:amount') || '', \
             localStorage.getItem('biltong:overrides') || ''])",
        );
        if let Ok(vals) = eval.recv::<Vec<String>>().await {
            let unit = vals.first().cloned().unwrap_or_default();
            let amount = vals.get(1).cloned().unwrap_or_default();
            let ov = vals.get(2).cloned().unwrap_or_default();
            if !ov.is_empty() {
                overrides.set(parse_overrides(&ov));
            }
            if !amount.is_empty() {
                raw.set(amount.clone());
                input.write().meat_grams = meat_to_grams(parse_amount(&amount), input().system);
            }
            if unit == "imperial" {
                set_system(input, &raw(), UnitSystem::Imperial);
            }
        }
    });

    let sys = input().system;
    let amount = parse_amount(&raw());
    let lines = compute(input().meat_grams, sys, &overrides());

    rsx! {
        section { id: "calculator", class: "bg-biltong-50 dark:bg-stone-900 py-16",
            div { class: "max-w-2xl mx-auto px-6",
                h2 { class: "font-display text-3xl font-bold text-biltong-700 dark:text-biltong-300 text-center mb-3",
                    "Recipe Calculator"
                }
                p { class: "text-center text-stone-600 dark:text-stone-300 mb-8",
                    "Enter how much beef you're starting with and the spice cure scales to match."
                }

                div { class: "bg-white dark:bg-stone-800 rounded-2xl shadow-sm ring-1 ring-biltong-100 dark:ring-stone-700 p-6 sm:p-8",
                    // Controls
                    div { class: "mb-6 space-y-4",
                        // Unit toggle — on top, since it drives the unit shown everywhere below
                        div {
                            span { class: "block text-sm font-semibold text-stone-700 dark:text-stone-200 mb-1", "Units" }
                            div { class: "flex w-full sm:inline-flex sm:w-auto rounded-lg ring-1 ring-biltong-300 dark:ring-stone-600 overflow-hidden",
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
                            span { class: "block text-sm font-semibold text-stone-700 dark:text-stone-200 mb-1",
                                "Amount of beef"
                            }
                            div { class: "flex rounded-lg ring-1 ring-biltong-300 dark:ring-stone-600 overflow-hidden",
                                button {
                                    r#type: "button",
                                    "aria-label": "Decrease amount",
                                    class: "px-4 bg-biltong-50 dark:bg-stone-700 text-biltong-700 dark:text-biltong-100 \
                                            text-xl font-bold leading-none hover:bg-biltong-100 dark:hover:bg-stone-600 transition-colors",
                                    onclick: move |_| step_amount(input, raw, -1.0),
                                    "−"
                                }
                                input {
                                    r#type: "number",
                                    min: "0",
                                    step: "0.1",
                                    inputmode: "decimal",
                                    value: "{raw}",
                                    class: "w-full min-w-0 border-0 bg-transparent text-stone-800 dark:text-stone-100 \
                                            px-3 py-2 text-center focus:outline-none focus:ring-2 focus:ring-inset focus:ring-biltong-500",
                                    oninput: move |evt| {
                                        raw.set(evt.value());
                                        let grams = meat_to_grams(parse_amount(&evt.value()), input().system);
                                        input.write().meat_grams = grams;
                                        persist_amount(&evt.value());
                                    },
                                }
                                span { class: "inline-flex items-center px-3 bg-biltong-50 dark:bg-stone-700 \
                                               text-stone-600 dark:text-stone-300 font-medium border-l border-biltong-300 dark:border-stone-600",
                                    "{sys.meat_unit()}"
                                }
                                button {
                                    r#type: "button",
                                    "aria-label": "Increase amount",
                                    class: "px-4 bg-biltong-50 dark:bg-stone-700 text-biltong-700 dark:text-biltong-100 \
                                            text-xl font-bold leading-none hover:bg-biltong-100 dark:hover:bg-stone-600 transition-colors \
                                            border-l border-biltong-300 dark:border-stone-600",
                                    onclick: move |_| step_amount(input, raw, 1.0),
                                    "+"
                                }
                            }
                        }
                        // Quick presets in the current unit
                        div {
                            span { class: "block text-sm font-semibold text-stone-700 dark:text-stone-200 mb-1", "Quick amounts" }
                            div { class: "flex flex-wrap gap-2",
                                for n in [1.0_f64, 2.0, 3.0, 4.0, 5.0] {
                                    button {
                                        key: "{n}",
                                        r#type: "button",
                                        class: if (amount - n).abs() < 1e-9 {
                                            "px-3 py-1.5 rounded-lg text-sm font-semibold border bg-biltong-700 text-white border-biltong-700"
                                        } else {
                                            "px-3 py-1.5 rounded-lg text-sm font-medium border border-biltong-300 dark:border-stone-600 \
                                             text-stone-700 dark:text-stone-300 hover:bg-biltong-50 dark:hover:bg-stone-700 transition-colors"
                                        },
                                        onclick: move |_| set_amount(input, raw, n),
                                        "{n} {sys.meat_unit()}"
                                    }
                                }
                            }
                        }
                    }

                    // Results — each amount is editable; an edit becomes that ingredient's
                    // custom ratio (remembered), so it keeps scaling with the meat weight.
                    if amount > 0.0 {
                        ul { class: "divide-y divide-biltong-100 dark:divide-stone-700",
                            for line in lines {
                                {
                                    let name = line.name;
                                    let is_vol = line.is_volume;
                                    let shown = edits().get(name).cloned().unwrap_or_else(|| line.value_str());
                                    rsx! {
                                        li { key: "{name}", class: "flex items-baseline justify-between gap-4 py-2.5",
                                            span { class: "min-w-0",
                                                span { class: "font-medium text-stone-800 dark:text-stone-100", "{name}" }
                                                if !line.note.is_empty() {
                                                    span { class: "block text-xs text-stone-400 dark:text-stone-500", "{line.note}" }
                                                }
                                            }
                                            span { class: "flex items-baseline gap-1 shrink-0",
                                                input {
                                                    r#type: "number",
                                                    min: "0",
                                                    step: "0.1",
                                                    inputmode: "decimal",
                                                    "aria-label": "{name} amount",
                                                    value: "{shown}",
                                                    class: if line.overridden {
                                                        "w-16 text-right bg-transparent font-semibold text-biltong-700 dark:text-biltong-300 \
                                                         border-b border-biltong-400 focus:outline-none focus:border-biltong-600"
                                                    } else {
                                                        "w-16 text-right bg-transparent font-semibold text-biltong-700 dark:text-biltong-300 \
                                                         border-b border-transparent hover:border-biltong-200 dark:hover:border-stone-600 focus:outline-none focus:border-biltong-500"
                                                    },
                                                    oninput: move |evt| {
                                                        let text = evt.value();
                                                        edits.write().insert(name.to_string(), text.clone());
                                                        if text.trim().is_empty() {
                                                            overrides.write().remove(name);
                                                        } else {
                                                            let f = factor_from_amount(
                                                                parse_amount(&text),
                                                                input().meat_grams,
                                                                input().system,
                                                                is_vol,
                                                            );
                                                            overrides.write().insert(name.to_string(), f);
                                                        }
                                                        persist_overrides(&overrides());
                                                    },
                                                    onfocusout: move |_| {
                                                        edits.write().remove(name);
                                                    },
                                                }
                                                span { class: "text-sm text-stone-500 dark:text-stone-400 w-9", "{line.unit}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !overrides().is_empty() {
                            div { class: "mt-3 text-right",
                                button {
                                    r#type: "button",
                                    class: "text-sm font-medium text-biltong-700 dark:text-biltong-300 hover:underline",
                                    onclick: move |_| {
                                        overrides.write().clear();
                                        edits.write().clear();
                                        persist_overrides(&overrides());
                                    },
                                    "↺ Reset to recipe amounts"
                                }
                            }
                        }
                    } else {
                        p { class: "text-center text-stone-400 dark:text-stone-500 py-6",
                            "Enter an amount of beef to see the spice cure."
                        }
                    }
                }
                p { class: "text-xs text-stone-400 dark:text-stone-500 text-center mt-4",
                    "Tap any amount to tweak it to taste — your edits scale with the weight and are remembered."
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
    raw.set(text.clone());
    input.write().meat_grams = meat_to_grams(next, sys);
    persist_amount(&text);
}

/// Set the meat amount directly (used by the preset chips).
fn set_amount(mut input: Signal<CalcInput>, mut raw: Signal<String>, value: f64) {
    let sys = input().system;
    let text = format!("{value}");
    raw.set(text.clone());
    input.write().meat_grams = meat_to_grams(value, sys);
    persist_amount(&text);
}

/// Remember the meat amount for next time (escaped so it can't break the JS string).
fn persist_amount(value: &str) {
    let safe = value.replace('\\', "\\\\").replace('\'', "\\'");
    document::eval(&format!(
        "try {{ localStorage.setItem('biltong:amount', '{safe}'); }} catch (e) {{}}"
    ));
}

/// Remember the ingredient overrides as a "name=factor;name=factor" string. Ingredient
/// names contain no `=`/`;`, so this round-trips cleanly with [`parse_overrides`].
fn persist_overrides(map: &Overrides) {
    let joined = map
        .iter()
        .map(|(name, factor)| format!("{name}={factor}"))
        .collect::<Vec<_>>()
        .join(";");
    let safe = joined.replace('\\', "\\\\").replace('\'', "\\'");
    document::eval(&format!(
        "try {{ localStorage.setItem('biltong:overrides', '{safe}'); }} catch (e) {{}}"
    ));
}

/// Parse the persisted "name=factor;…" string back into an overrides map.
fn parse_overrides(stored: &str) -> Overrides {
    let mut map = Overrides::new();
    for part in stored.split(';') {
        if let Some((name, factor)) = part.rsplit_once('=') {
            if let Ok(f) = factor.parse::<f64>() {
                map.insert(name.to_string(), f);
            }
        }
    }
    map
}

/// Switch unit system, re-deriving the stored grams from the current raw input, and
/// remember the choice for next time.
fn set_system(mut input: Signal<CalcInput>, raw: &str, system: UnitSystem) {
    {
        let mut st = input.write();
        st.system = system;
        st.meat_grams = meat_to_grams(parse_amount(raw), system);
    }
    let value = match system {
        UnitSystem::Metric => "metric",
        UnitSystem::Imperial => "imperial",
    };
    document::eval(&format!(
        "try {{ localStorage.setItem('biltong:unit', '{value}'); }} catch (e) {{}}"
    ));
}

#[component]
fn UnitButton(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let classes = if active {
        "bg-biltong-700 text-biltong-50"
    } else {
        "bg-white dark:bg-stone-800 text-stone-600 dark:text-stone-300 hover:bg-biltong-50 dark:hover:bg-stone-700"
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
