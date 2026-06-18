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
            h2 { class: "font-display text-3xl font-bold text-biltong-700 dark:text-biltong-300 text-center mb-12",
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

/// Scroll-spy for the anchor rail. A first observer reports the step nearest the middle of
/// the viewport (a thin band near the top-centre) so the rail can highlight it; a second
/// observer tracks whether the steps section is on screen at all, sending `-1` to hide the
/// rail (so it never floats over the hero/footer, whose dark background matches the marker).
const SCROLL_SPY_JS: &str = r#"
    let active = 0;
    const band = new IntersectionObserver((entries) => {
        for (const e of entries) {
            if (e.isIntersecting) { active = parseInt(e.target.dataset.step, 10); dioxus.send(active); }
        }
    }, { rootMargin: '-40% 0px -55% 0px', threshold: 0 });
    document.querySelectorAll('[data-step]').forEach((el) => band.observe(el));

    const section = document.getElementById('steps');
    const vis = new IntersectionObserver((entries) => {
        for (const e of entries) { dioxus.send(e.isIntersecting ? active : -1); }
    }, { threshold: 0 });
    if (section) vis.observe(section);
"#;

/// Ant-Design-style anchor: a vertical rail (numbered dots, with step titles on wide
/// screens) that highlights the step currently in view and jumps to it on click. It is
/// shown only while the steps section is on screen.
#[component]
fn StepAnchor() -> Element {
    let mut active = use_signal(|| 0usize);
    let mut visible = use_signal(|| false);
    use_future(move || async move {
        let mut eval = document::eval(SCROLL_SPY_JS);
        while let Ok(i) = eval.recv::<i64>().await {
            if i < 0 {
                visible.set(false);
            } else {
                visible.set(true);
                active.set(i as usize);
            }
        }
    });

    rsx! {
        nav {
            "aria-label": "Steps",
            class: if visible() {
                "fixed right-2 sm:right-3 top-1/2 -translate-y-1/2 z-40 transition-opacity duration-300 opacity-100"
            } else {
                "fixed right-2 sm:right-3 top-1/2 -translate-y-1/2 z-40 transition-opacity duration-300 opacity-0 pointer-events-none"
            },
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
                                    "hidden xl:block text-sm max-w-[11rem] truncate transition-colors text-biltong-700 dark:text-biltong-300 font-semibold"
                                } else {
                                    "hidden xl:block text-sm max-w-[11rem] truncate transition-colors text-stone-400 group-hover:text-biltong-600"
                                },
                                "{i + 1}. {step.title}"
                            }
                            // numbered dot — always visible; same size so the rail stays aligned
                            span {
                                class: if active() == i {
                                    "flex items-center justify-center w-5 h-5 rounded-full text-[10px] font-bold \
                                     transition-colors bg-biltong-700 text-biltong-50"
                                } else {
                                    "flex items-center justify-center w-5 h-5 rounded-full text-[10px] font-bold \
                                     transition-colors bg-biltong-100 text-biltong-700 border border-biltong-300 \
                                     group-hover:bg-biltong-300"
                                },
                                "{i + 1}"
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
            class: "grid sm:grid-cols-2 gap-6 items-center bg-white dark:bg-stone-800 rounded-2xl \
                    shadow-sm ring-1 ring-biltong-100 dark:ring-stone-700 p-6 scroll-mt-20",
            div { class: "bg-biltong-50 dark:bg-stone-900 rounded-xl p-2 {media_order}",
                StepArt { kind: step.anim }
            }
            div { class: "{text_order}",
                div { class: "flex items-center gap-3 mb-2",
                    span { class: "flex items-center justify-center w-9 h-9 rounded-full \
                                   bg-biltong-700 text-biltong-50 font-bold",
                        "{index + 1}"
                    }
                    h3 { class: "font-display text-2xl font-semibold text-biltong-900 dark:text-biltong-100",
                        "{step.title}"
                    }
                }
                p { class: "text-stone-600 dark:text-stone-300 leading-relaxed", "{body}" }
            }
        }
    }
}
