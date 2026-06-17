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
            class: "hidden xl:block fixed right-4 top-1/2 -translate-y-1/2 z-40",
            ul { class: "flex flex-col text-sm",
                for (i , step) in STEPS.iter().enumerate() {
                    li { key: "{i}",
                        a {
                            href: "#step-{i}",
                            title: "{step.title}",
                            class: if active() == i {
                                "block border-l-2 pl-3 py-1.5 max-w-[11rem] truncate transition-colors \
                                 border-biltong-700 text-biltong-700 font-semibold"
                            } else {
                                "block border-l-2 pl-3 py-1.5 max-w-[11rem] truncate transition-colors \
                                 border-biltong-100 text-stone-400 hover:text-biltong-600 hover:border-biltong-300"
                            },
                            "{i + 1}. {step.title}"
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
