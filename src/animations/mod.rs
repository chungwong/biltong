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
        AnimKind::Toast => rsx! { ToastArt {} },
        AnimKind::Grind => rsx! { GrindArt {} },
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

/// Step 1 — slicing the beef. The illustration emphasises the two things the text calls
/// out: cut WITH the grain (knife travels along the grain striations, parallel to the
/// cut lines) and keep each strip ~2 cm thick (a pulsing dimension marker).
#[component]
fn SliceArt() -> Element {
    rsx! {
        ArtFrame {
            // "cut with the grain" label + a direction arrow that runs along the grain
            text {
                x: "14",
                y: "15",
                font_size: "10",
                font_weight: "bold",
                fill: "#7c2d12",
                "cut with the grain"
            }
            line { x1: "16", y1: "24", x2: "120", y2: "24", stroke: "#a85a32", stroke_width: "2" }
            polygon { points: "120,20 130,24 120,28", fill: "#a85a32" }

            // cutting board
            rect { x: "16", y: "110", width: "150", height: "10", rx: "4", fill: "#d8a47f" }

            // meat as stacked ~2 cm strips; grain striations run along their length, so the
            // cuts between strips are parallel to the grain.
            for (i , y) in [38.0_f64, 60.0, 82.0].into_iter().enumerate() {
                g { key: "{i}",
                    rect { x: "30", y: "{y}", width: "112", height: "18", rx: "4", fill: "#7c2d12" }
                    line {
                        x1: "36",
                        y1: "{y + 6.0}",
                        x2: "136",
                        y2: "{y + 6.0}",
                        stroke: "#a85a32",
                        stroke_width: "1.5",
                        opacity: "0.6",
                    }
                    line {
                        x1: "36",
                        y1: "{y + 12.0}",
                        x2: "136",
                        y2: "{y + 12.0}",
                        stroke: "#a85a32",
                        stroke_width: "1.5",
                        opacity: "0.4",
                    }
                }
            }

            // thickness dimension on the top strip (y 38..56), gently pulsing for emphasis
            g { class: "anim-pulse",
                line { x1: "150", y1: "38", x2: "162", y2: "38", stroke: "#4a1505", stroke_width: "1.5" }
                line { x1: "150", y1: "56", x2: "162", y2: "56", stroke: "#4a1505", stroke_width: "1.5" }
                line { x1: "156", y1: "38", x2: "156", y2: "56", stroke: "#4a1505", stroke_width: "1.5" }
                text {
                    x: "165",
                    y: "50",
                    font_size: "9",
                    font_weight: "bold",
                    fill: "#4a1505",
                    "2 cm"
                }
            }

            // the knife slices horizontally — i.e. along the grain — through a cut line
            g { class: "anim-slice",
                rect { x: "46", y: "55", width: "92", height: "6", rx: "2", fill: "#cbd5e1" }
                polygon { points: "46,55 38,58 46,61", fill: "#cbd5e1" }
                rect { x: "138", y: "52", width: "8", height: "12", rx: "2", fill: "#9ca3af" }
                rect { x: "146", y: "51", width: "26", height: "14", rx: "3", fill: "#4a1505" }
            }
        }
    }
}

/// Step 2 — the two sauces (red wine vinegar + Worcestershire) raining onto a tray of
/// sliced beef seen from above (orthogonal view).
#[component]
fn VinegarArt() -> Element {
    rsx! {
        ArtFrame {
            // --- tray of sliced beef, top-down ---
            rect {
                x: "18",
                y: "72",
                width: "164",
                height: "48",
                rx: "6",
                fill: "#c9ad93",
                stroke: "#a98a6f",
                stroke_width: "2",
            }
            for (i , x) in [28.0_f64, 58.0, 88.0, 118.0, 148.0].into_iter().enumerate() {
                g { key: "{i}",
                    rect { x: "{x}", y: "78", width: "20", height: "36", rx: "4", fill: "#7c2d12" }
                    line {
                        x1: "{x + 10.0}",
                        y1: "82",
                        x2: "{x + 10.0}",
                        y2: "110",
                        stroke: "#a85a32",
                        stroke_width: "1.5",
                        opacity: "0.5",
                    }
                }
            }

            // --- red wine vinegar bottle (left) ---
            rect { x: "50", y: "14", width: "24", height: "30", rx: "5", fill: "#7f1d1d" }
            rect { x: "53", y: "22", width: "18", height: "10", rx: "1", fill: "#fdf6f0", opacity: "0.9" }
            rect { x: "59", y: "44", width: "6", height: "5", fill: "#450a0a" }
            // red droplets
            circle { class: "anim-drop", cx: "62", cy: "50", r: "3.5", fill: "#b91c1c",
                style: "animation-delay: 0s" }
            circle { class: "anim-drop", cx: "59", cy: "50", r: "3", fill: "#b91c1c",
                style: "animation-delay: 0.6s" }
            circle { class: "anim-drop", cx: "65", cy: "50", r: "3", fill: "#b91c1c",
                style: "animation-delay: 1.2s" }

            // --- Worcestershire bottle (right) ---
            rect { x: "116", y: "14", width: "24", height: "30", rx: "5", fill: "#3f2d1a" }
            rect { x: "119", y: "22", width: "18", height: "10", rx: "1", fill: "#fde68a", opacity: "0.9" }
            rect { x: "120", y: "8", width: "16", height: "6", rx: "1", fill: "#f59e0b" }
            rect { x: "125", y: "44", width: "6", height: "5", fill: "#1f1408" }
            // dark-brown droplets
            circle { class: "anim-drop", cx: "128", cy: "50", r: "3.5", fill: "#3f2d1a",
                style: "animation-delay: 0.3s" }
            circle { class: "anim-drop", cx: "125", cy: "50", r: "3", fill: "#3f2d1a",
                style: "animation-delay: 0.9s" }
            circle { class: "anim-drop", cx: "131", cy: "50", r: "3", fill: "#3f2d1a",
                style: "animation-delay: 1.5s" }

            // --- legend ---
            rect { x: "18", y: "126", width: "9", height: "7", rx: "1", fill: "#b91c1c" }
            text { x: "30", y: "132", font_size: "7", fill: "#4a1505", "red wine vinegar" }
            rect { x: "104", y: "126", width: "9", height: "7", rx: "1", fill: "#3f2d1a" }
            text { x: "116", y: "132", font_size: "7", fill: "#4a1505", "Worcestershire" }
        }
    }
}

/// Step 3 — toasting the coriander seeds in a dry pan over a flame.
#[component]
fn ToastArt() -> Element {
    rsx! {
        ArtFrame {
            // flames under the pan (flickering)
            for (i , cx) in [74.0_f64, 90.0, 106.0].into_iter().enumerate() {
                g { key: "f{i}", class: "anim-jiggle", style: "animation-delay: {i as f64 * 0.2}s",
                    path { d: "M{cx} 118 q-7 -11 0 -22 q7 13 0 22 z", fill: "#f97316" }
                    path { d: "M{cx} 116 q-3 -7 0 -14 q3 9 0 14 z", fill: "#fde047" }
                }
            }
            // pan
            ellipse { cx: "90", cy: "84", rx: "56", ry: "13", fill: "#6b7280" }
            ellipse { cx: "90", cy: "81", rx: "49", ry: "10", fill: "#3f4754" }
            rect { x: "142", y: "79", width: "50", height: "6", rx: "3", fill: "#374151" }
            // coriander seeds toasting (jiggling)
            for (i , (cx , cy)) in [
                (66.0_f64, 82.0_f64),
                (80.0, 79.0),
                (94.0, 82.0),
                (106.0, 80.0),
                (74.0, 84.0),
                (100.0, 84.0),
            ]
                .into_iter()
                .enumerate()
            {
                ellipse {
                    key: "s{i}",
                    class: "anim-jiggle",
                    style: "animation-delay: {i as f64 * 0.15}s",
                    cx: "{cx}",
                    cy: "{cy}",
                    rx: "3",
                    ry: "2.5",
                    fill: "#c9a36a",
                }
            }
            // heat shimmer rising off the pan
            for (i , x) in [72.0_f64, 90.0, 108.0].into_iter().enumerate() {
                path {
                    key: "h{i}",
                    class: "anim-rise",
                    d: "M{x} 70 q5 -5 0 -10 q-5 -5 0 -10",
                    fill: "none",
                    stroke: "#f59e0b",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    opacity: "0.7",
                    style: "animation-delay: {i as f64 * 0.4}s",
                }
            }
        }
    }
}

/// Step 4 — grinding the toasted spices in a mortar and pestle.
#[component]
fn GrindArt() -> Element {
    rsx! {
        ArtFrame {
            // mortar bowl
            ellipse { cx: "100", cy: "76", rx: "58", ry: "11", fill: "#c9ad93" }
            path { d: "M44 76 Q100 134 156 76 Z", fill: "#d8a47f" }
            ellipse { cx: "100", cy: "76", rx: "47", ry: "8", fill: "#6b5a48" }
            // toasted seeds / powder in the cavity
            for (i , (cx , cy)) in [(86.0_f64, 78.0_f64), (104.0, 79.0), (96.0, 80.0), (112.0, 77.0)]
                .into_iter()
                .enumerate()
            {
                ellipse { key: "g{i}", cx: "{cx}", cy: "{cy}", rx: "2.5", ry: "2", fill: "#c9a36a" }
            }
            // pestle, rocking as it grinds (pivots at the hand end, top of the bbox)
            g { class: "anim-sway",
                rect { x: "96", y: "18", width: "10", height: "50", rx: "5", fill: "#cbb89f" }
                ellipse { cx: "101", cy: "74", rx: "11", ry: "9", fill: "#b9a589" }
            }
        }
    }
}

/// Step 5 — spice being sprinkled over the meat.
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
