//! The tutorial steps. Each step is a card with its number, copy and an animated SVG,
//! alternating the illustration left/right on wider screens.

use crate::animations::StepArt;
use crate::calculator::{drying_temp, slice_thickness, CalcInput};
use crate::recipe::{Step, STEPS};
use dioxus::prelude::*;

#[component]
pub fn Steps() -> Element {
    rsx! {
        section { id: "steps", class: "max-w-4xl mx-auto px-6 py-16",
            StepAnchor {}
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

/// Scroll-spy that reports the index of the step nearest the middle of the viewport, so the
/// anchor rail can highlight it. Uses an `IntersectionObserver` with a thin band near the
/// top-centre; whenever a step enters that band it posts its index back to Rust.
const SCROLL_SPY_JS: &str = r#"
    let current = -1;
    const send = (i) => { if (i !== current) { current = i; dioxus.send(i); } };
    const obs = new IntersectionObserver((entries) => {
        for (const e of entries) {
            if (e.isIntersecting) send(parseInt(e.target.dataset.step, 10));
        }
    }, { rootMargin: '-40% 0px -55% 0px', threshold: 0 });
    document.querySelectorAll('[data-step]').forEach((el) => obs.observe(el));
"#;

/// Ant-Design-style anchor: a fixed vertical rail (wide screens only) that lists the steps
/// and highlights whichever is currently in view; clicking jumps to that step.
#[component]
fn StepAnchor() -> Element {
    let mut active = use_signal(|| 0usize);
    use_future(move || async move {
        let mut eval = document::eval(SCROLL_SPY_JS);
        while let Ok(i) = eval.recv::<i64>().await {
            active.set(i.max(0) as usize);
        }
    });

    rsx! {
        nav {
            "aria-label": "Steps",
            class: "fixed right-2 sm:right-3 top-1/2 -translate-y-1/2 z-40",
            ul { class: "flex flex-col gap-1.5 items-end",
                for (i , step) in STEPS.iter().enumerate() {
                    li { key: "{i}",
                        a {
                            href: "#step-{i}",
                            title: "{step.title}",
                            "aria-label": "{step.title}",
                            class: "group flex items-center gap-2 py-1 pl-3",
                            // label — only on wide screens
                            span {
                                class: if active() == i {
                                    "hidden xl:block text-sm max-w-[11rem] truncate transition-colors text-biltong-700 font-semibold"
                                } else {
                                    "hidden xl:block text-sm max-w-[11rem] truncate transition-colors text-stone-400 group-hover:text-biltong-600"
                                },
                                "{i + 1}. {step.title}"
                            }
                            // dot — always visible
                            span {
                                class: if active() == i {
                                    "block w-3 h-3 rounded-full bg-biltong-700 ring-2 ring-biltong-300 transition-all"
                                } else {
                                    "block w-2 h-2 rounded-full bg-biltong-300 group-hover:bg-biltong-500 transition-all"
                                },
                            }
                        }
                    }
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

    // Substitute the unit-aware values into the body (no-op for steps without them).
    let system = use_context::<Signal<CalcInput>>()().system;
    let body = step
        .body
        .replace("{temp}", drying_temp(system))
        .replace("{thickness}", slice_thickness(system));

    rsx! {
        article {
            id: "step-{index}",
            "data-step": "{index}",
            class: "grid sm:grid-cols-2 gap-6 items-center bg-white rounded-2xl \
                    shadow-sm ring-1 ring-biltong-100 p-6 scroll-mt-20",
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
