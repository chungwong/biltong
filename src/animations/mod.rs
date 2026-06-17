//! Inline-SVG illustrations for each step, animated purely by the CSS keyframes defined
//! in `tailwind.css`. Each step gets its own small component; [`StepArt`] dispatches to
//! the right one based on the step's [`AnimKind`].

use crate::recipe::AnimKind;
use dioxus::prelude::*;

/// Render the animation that illustrates a given step.
#[component]
pub fn StepArt(kind: AnimKind) -> Element {
    match kind {
        AnimKind::Slice => rsx! { SliceArt {} },
        AnimKind::Vinegar => rsx! { VinegarArt {} },
        AnimKind::Spice => rsx! { SpiceArt {} },
        AnimKind::Cure => rsx! { CureArt {} },
        AnimKind::Dry => rsx! { DryArt {} },
        AnimKind::Done => rsx! { DoneArt {} },
    }
}

/// Shared wrapper so every illustration sits in a consistent, responsive box.
#[component]
fn ArtFrame(children: Element) -> Element {
    rsx! {
        svg {
            class: "w-full h-40 sm:h-48",
            view_box: "0 0 200 140",
            role: "img",
            {children}
        }
    }
}

/// Step 1 — a knife slicing a block of meat.
#[component]
fn SliceArt() -> Element {
    rsx! {
        ArtFrame {
            // cutting board
            rect { x: "20", y: "96", width: "160", height: "12", rx: "4", fill: "#d8a47f" }
            // meat block
            rect { x: "48", y: "62", width: "104", height: "36", rx: "10", fill: "#7c2d12" }
            rect { x: "48", y: "62", width: "26", height: "36", rx: "10", fill: "#a85a32", opacity: "0.6" }
            // pre-cut lines
            line { x1: "82", y1: "64", x2: "82", y2: "96", stroke: "#4a1505", stroke_width: "2" }
            line { x1: "104", y1: "64", x2: "104", y2: "96", stroke: "#4a1505", stroke_width: "2" }
            line { x1: "126", y1: "64", x2: "126", y2: "96", stroke: "#4a1505", stroke_width: "2" }
            // the knife (animated)
            g { class: "anim-knife",
                rect { x: "120", y: "20", width: "60", height: "10", rx: "3", fill: "#9ca3af" }
                rect { x: "172", y: "16", width: "20", height: "18", rx: "4", fill: "#4a1505" }
            }
        }
    }
}

/// Step 2 — vinegar droplets falling onto the strips.
#[component]
fn VinegarArt() -> Element {
    rsx! {
        ArtFrame {
            // bowl of strips
            path { d: "M40 96 Q100 132 160 96 L160 86 Q100 110 40 86 Z", fill: "#a85a32" }
            rect { x: "54", y: "80", width: "92", height: "12", rx: "6", fill: "#7c2d12" }
            // bottle
            rect { x: "150", y: "18", width: "26", height: "40", rx: "5", fill: "#4d7c0f" }
            rect { x: "158", y: "8", width: "10", height: "12", rx: "2", fill: "#3f6212" }
            // falling droplets (animated, staggered)
            circle { class: "anim-drop", cx: "92", cy: "44", r: "4", fill: "#b45309",
                style: "animation-delay: 0s" }
            circle { class: "anim-drop", cx: "108", cy: "44", r: "3.5", fill: "#b45309",
                style: "animation-delay: 0.5s" }
            circle { class: "anim-drop", cx: "100", cy: "44", r: "3", fill: "#b45309",
                style: "animation-delay: 1s" }
        }
    }
}

/// Step 3 — spice being sprinkled over the meat.
#[component]
fn SpiceArt() -> Element {
    rsx! {
        ArtFrame {
            // meat strips
            rect { x: "44", y: "92", width: "112", height: "14", rx: "7", fill: "#7c2d12" }
            rect { x: "52", y: "78", width: "96", height: "14", rx: "7", fill: "#7c2d12" }
            // spice shaker
            rect { x: "146", y: "16", width: "30", height: "30", rx: "5", fill: "#4a1505" }
            rect { x: "150", y: "8", width: "22", height: "10", rx: "3", fill: "#a85a32" }
            // sprinkling spice flecks (animated, staggered)
            rect { class: "anim-sprinkle", x: "80", y: "44", width: "4", height: "4", fill: "#b45309",
                style: "animation-delay: 0s" }
            rect { class: "anim-sprinkle", x: "96", y: "44", width: "3", height: "3", fill: "#4d7c0f",
                style: "animation-delay: 0.4s" }
            rect { class: "anim-sprinkle", x: "112", y: "44", width: "4", height: "4", fill: "#7c2d12",
                style: "animation-delay: 0.8s" }
            rect { class: "anim-sprinkle", x: "88", y: "44", width: "3", height: "3", fill: "#b45309",
                style: "animation-delay: 1.2s" }
        }
    }
}

/// Step 4 — resting in the fridge: a ticking clock.
#[component]
fn CureArt() -> Element {
    rsx! {
        ArtFrame {
            // container of curing meat
            rect { x: "26", y: "70", width: "70", height: "44", rx: "8", fill: "#d8a47f" }
            rect { x: "34", y: "78", width: "54", height: "10", rx: "5", fill: "#7c2d12" }
            rect { x: "34", y: "92", width: "54", height: "10", rx: "5", fill: "#7c2d12" }
            // clock
            circle { cx: "146", cy: "70", r: "34", fill: "#fdf6f0", stroke: "#4a1505", stroke_width: "4" }
            circle { cx: "146", cy: "70", r: "3", fill: "#4a1505" }
            // rotating hand (animated)
            g { class: "anim-tick",
                line { x1: "146", y1: "70", x2: "146", y2: "46", stroke: "#7c2d12", stroke_width: "4",
                    stroke_linecap: "round" }
            }
            line { x1: "146", y1: "70", x2: "164", y2: "70", stroke: "#a85a32", stroke_width: "3",
                stroke_linecap: "round" }
        }
    }
}

/// Step 5 — strips hanging and drying with airflow.
#[component]
fn DryArt() -> Element {
    rsx! {
        ArtFrame {
            // hanging rail
            line { x1: "20", y1: "24", x2: "180", y2: "24", stroke: "#4a1505", stroke_width: "4",
                stroke_linecap: "round" }
            // swaying strips (animated, staggered)
            for (i , x) in [70.0_f64, 100.0, 130.0].into_iter().enumerate() {
                g { key: "{i}", class: "anim-sway",
                    style: "transform-origin: {x}px 24px; animation-delay: {i as f64 * 0.4}s",
                    line { x1: "{x}", y1: "24", x2: "{x}", y2: "40", stroke: "#9ca3af", stroke_width: "2" }
                    rect { x: "{x - 9.0}", y: "40", width: "18", height: "58", rx: "9", fill: "#7c2d12" }
                    ellipse { cx: "{x}", cy: "54", rx: "5", ry: "3", fill: "#a85a32", opacity: "0.6" }
                }
            }
            // fan airflow lines (animated, staggered)
            for (i , y) in [56.0_f64, 74.0, 92.0].into_iter().enumerate() {
                path { key: "f{i}", class: "anim-airflow",
                    d: "M18 {y} q10 -6 20 0",
                    fill: "none", stroke: "#4d7c0f", stroke_width: "2", stroke_linecap: "round",
                    style: "animation-delay: {i as f64 * 0.3}s" }
            }
        }
    }
}

/// Step 6 — the finished biltong, sliced and fanned out.
#[component]
fn DoneArt() -> Element {
    rsx! {
        ArtFrame {
            // plate
            ellipse { cx: "100", cy: "104", rx: "76", ry: "14", fill: "#d8a47f" }
            // fanned slices (animated in, staggered)
            for (i , x) in [62.0_f64, 80.0, 98.0, 116.0, 134.0].into_iter().enumerate() {
                g { key: "{i}", class: "anim-fan-in", style: "animation-delay: {i as f64 * 0.15}s",
                    ellipse { cx: "{x}", cy: "78", rx: "13", ry: "20", fill: "#7c2d12",
                        transform: "rotate({(i as f64 - 2.0) * 8.0} {x} 78)" }
                    ellipse { cx: "{x}", cy: "78", rx: "7", ry: "13", fill: "#a85a32", opacity: "0.5",
                        transform: "rotate({(i as f64 - 2.0) * 8.0} {x} 78)" }
                }
            }
            // gentle shine
            circle { class: "anim-shine", cx: "100", cy: "60", r: "40", fill: "#fdf6f0" }
        }
    }
}
