//! Inline-SVG illustrations for each step, animated purely by the CSS keyframes defined
//! in `tailwind.css`. Each step gets its own small component; [`StepArt`] dispatches to
//! the right one based on the step's [`AnimKind`].

use crate::calculator::{
    amount_of, drying_temp, format_meat, slice_thickness, spice_blend_amount, CalcInput, Overrides,
};
use crate::recipe::AnimKind;
use dioxus::prelude::*;

/// Read the shared calculator input so an animation can show live amounts.
fn calc_input() -> CalcInput {
    use_context::<Signal<CalcInput>>()()
}

/// Read the shared ingredient overrides so animation amounts match the calculator.
fn calc_overrides() -> Overrides {
    use_context::<Signal<Overrides>>()()
}

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
            class: "w-full h-40 sm:h-48 art-svg",
            view_box: "0 0 200 140",
            role: "img",
            {children}
        }
    }
}

/// A single biltong strip seen side-on, in the same dark, sheened look as the hanging
/// strips in step 7 — so the whole strips read consistently across steps 5–7. (The sliced
/// cross-section look — red interior + fat flecks — is reserved for step 8.) `(cx, cy)` is
/// the centre; `hl`/`ht` are the half-length and half-thickness.
#[component]
fn MeatStrip(cx: f64, cy: f64, hl: f64, ht: f64) -> Element {
    rsx! {
        rect { x: "{cx - hl}", y: "{cy - ht}", width: "{2.0 * hl}", height: "{2.0 * ht}", rx: "{ht}", fill: "#54200f" }
        ellipse { cx: "{cx}", cy: "{cy - ht * 0.3}", rx: "{hl * 0.82}", ry: "{ht * 0.42}", fill: "#6e2c18", opacity: "0.85" }
        ellipse { cx: "{cx - hl * 0.15}", cy: "{cy + ht * 0.15}", rx: "{hl * 0.5}", ry: "{ht * 0.25}", fill: "#8a3a26", opacity: "0.55" }
    }
}

/// Step 1 — slicing the beef. The illustration emphasises the two things the text calls
/// out: cut WITH the grain (knife travels along the grain striations, parallel to the
/// cut lines) and keep each strip ~2 cm thick (a pulsing dimension marker).
#[component]
fn SliceArt() -> Element {
    let input = calc_input();
    let meat = format_meat(input.meat_grams, input.system);
    let thickness = slice_thickness(input.system);
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
            // live meat weight from the calculator
            text {
                x: "186",
                y: "15",
                text_anchor: "end",
                font_size: "10",
                font_weight: "bold",
                fill: "#4a1505",
                "{meat} beef"
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
                line { x1: "150", y1: "38", x2: "162", y2: "38", stroke: "currentColor", stroke_width: "1.5" }
                line { x1: "150", y1: "56", x2: "162", y2: "56", stroke: "currentColor", stroke_width: "1.5" }
                line { x1: "156", y1: "38", x2: "156", y2: "56", stroke: "currentColor", stroke_width: "1.5" }
                text {
                    x: "165",
                    y: "50",
                    font_size: "9",
                    font_weight: "bold",
                    fill: "#4a1505",
                    "{thickness}"
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
    let input = calc_input();
    let ov = calc_overrides();
    let vinegar = amount_of("Red wine vinegar", input.meat_grams, input.system, &ov);
    let worcester = amount_of("Worcestershire sauce", input.meat_grams, input.system, &ov);
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

            // --- red wine vinegar bottle (left) with its amount beside it ---
            rect { x: "50", y: "14", width: "24", height: "30", rx: "5", fill: "#7f1d1d" }
            rect { x: "53", y: "22", width: "18", height: "10", rx: "1", fill: "#fdf6f0", opacity: "0.9" }
            rect { x: "59", y: "44", width: "6", height: "5", fill: "#450a0a" }
            text { x: "46", y: "24", text_anchor: "end", font_size: "7", fill: "#7f1d1d", "red wine" }
            text { x: "46", y: "33", text_anchor: "end", font_size: "7", fill: "#7f1d1d", "vinegar" }
            text { x: "46", y: "45", text_anchor: "end", font_size: "9", font_weight: "bold", fill: "#4a1505",
                "{vinegar}" }
            // red droplets
            circle { class: "anim-drop", cx: "62", cy: "50", r: "3.5", fill: "#b91c1c",
                style: "animation-delay: 0s" }
            circle { class: "anim-drop", cx: "59", cy: "50", r: "3", fill: "#b91c1c",
                style: "animation-delay: 0.6s" }
            circle { class: "anim-drop", cx: "65", cy: "50", r: "3", fill: "#b91c1c",
                style: "animation-delay: 1.2s" }

            // --- Worcestershire bottle (right) with its amount beside it ---
            rect { x: "116", y: "14", width: "24", height: "30", rx: "5", fill: "#3f2d1a" }
            rect { x: "119", y: "22", width: "18", height: "10", rx: "1", fill: "#fde68a", opacity: "0.9" }
            rect { x: "120", y: "8", width: "16", height: "6", rx: "1", fill: "#f59e0b" }
            rect { x: "125", y: "44", width: "6", height: "5", fill: "#1f1408" }
            text { x: "144", y: "24", font_size: "7", fill: "#3f2d1a", "Worcester-" }
            text { x: "144", y: "33", font_size: "7", fill: "#3f2d1a", "shire" }
            text { x: "144", y: "45", font_size: "9", font_weight: "bold", fill: "#4a1505", "{worcester}" }
            // dark-brown droplets
            circle { class: "anim-drop", cx: "128", cy: "50", r: "3.5", fill: "#3f2d1a",
                style: "animation-delay: 0.3s" }
            circle { class: "anim-drop", cx: "125", cy: "50", r: "3", fill: "#3f2d1a",
                style: "animation-delay: 0.9s" }
            circle { class: "anim-drop", cx: "131", cy: "50", r: "3", fill: "#3f2d1a",
                style: "animation-delay: 1.5s" }
        }
    }
}

/// Step 3 — toasting the coriander seeds in a dry pan over a flame.
#[component]
fn ToastArt() -> Element {
    let input = calc_input();
    let ov = calc_overrides();
    let coriander = amount_of(
        "Coriander seed (toasted)",
        input.meat_grams,
        input.system,
        &ov,
    );
    rsx! {
        ArtFrame {
            // live coriander amount
            text {
                x: "10",
                y: "14",
                font_size: "9",
                font_weight: "bold",
                fill: "#4a1505",
                "Coriander seed: {coriander}"
            }
            // toasting time — small clock + label, top-right
            circle { cx: "150", cy: "11", r: "6", fill: "none", stroke: "currentColor", stroke_width: "1.5" }
            line { x1: "150", y1: "11", x2: "150", y2: "7", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round" }
            line { x1: "150", y1: "11", x2: "153.5", y2: "11", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round" }
            text { x: "160", y: "14", font_size: "9", font_weight: "bold", fill: "#4a1505", "3–5 min" }
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
    let input = calc_input();
    let ov = calc_overrides();
    let coriander = amount_of(
        "Coriander seed (toasted)",
        input.meat_grams,
        input.system,
        &ov,
    );
    let pepper = amount_of("Peppercorns", input.meat_grams, input.system, &ov);
    let chili = amount_of("Chili flakes", input.meat_grams, input.system, &ov);
    rsx! {
        ArtFrame {
            // live spice amounts going into the blend
            text { x: "8", y: "12", font_size: "8", font_weight: "bold", fill: "#4a1505",
                "Coriander {coriander}" }
            text { x: "8", y: "23", font_size: "8", font_weight: "bold", fill: "#4a1505",
                "Pepper {pepper}" }
            text { x: "8", y: "34", font_size: "8", font_weight: "bold", fill: "#4a1505",
                "Chili {chili}" }
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

/// Step 5 — rubbing the salt and spice blend into the meat. A hand works back and forth
/// over a steak dusted with white salt grains and amber spice specks.
#[component]
fn SpiceArt() -> Element {
    let input = calc_input();
    let ov = calc_overrides();
    let salt = amount_of("Salt", input.meat_grams, input.system, &ov);
    let blend = spice_blend_amount(input.meat_grams, input.system, &ov);
    rsx! {
        ArtFrame {
            // three biltong strips being salted & spiced
            MeatStrip { cx: 100.0, cy: 60.0, hl: 60.0, ht: 8.0 }
            MeatStrip { cx: 100.0, cy: 80.0, hl: 60.0, ht: 8.0 }
            MeatStrip { cx: 100.0, cy: 100.0, hl: 60.0, ht: 8.0 }

            // salt grains (white) being worked in
            for (i , (x , y)) in [
                (50.0_f64, 64.0_f64),
                (70.0, 60.0),
                (120.0, 62.0),
                (140.0, 68.0),
                (60.0, 82.0),
                (132.0, 84.0),
            ]
                .into_iter()
                .enumerate()
            {
                rect {
                    key: "salt{i}",
                    class: "anim-jiggle",
                    style: "animation-delay: {i as f64 * 0.18}s",
                    x: "{x}",
                    y: "{y}",
                    width: "3",
                    height: "3",
                    fill: "#f8fafc",
                }
            }

            // spice specks (coriander amber + dark pepper)
            for (i , (cx , cy , c)) in [
                (58.0_f64, 74.0_f64, "#b45309"),
                (84.0, 64.0, "#4a1505"),
                (104.0, 62.0, "#b45309"),
                (150.0, 62.0, "#4a1505"),
                (74.0, 90.0, "#b45309"),
                (122.0, 76.0, "#4a1505"),
            ]
                .into_iter()
                .enumerate()
            {
                circle {
                    key: "sp{i}",
                    class: "anim-jiggle",
                    style: "animation-delay: {i as f64 * 0.22}s",
                    cx: "{cx}",
                    cy: "{cy}",
                    r: "2.5",
                    fill: "{c}",
                }
            }

            // hand rubbing the seasoning in (sweeps back and forth)
            g { class: "anim-slice",
                rect { x: "73", y: "50", width: "10", height: "7", rx: "3", fill: "#e8b48f" }
                rect { x: "80", y: "40", width: "40", height: "20", rx: "9", fill: "#e8b48f" }
                rect { x: "80", y: "40", width: "40", height: "6", rx: "9", fill: "#d99c73", opacity: "0.5" }
                for (i , fx) in [82.0_f64, 91.0, 100.0, 109.0].into_iter().enumerate() {
                    rect { key: "fg{i}", x: "{fx}", y: "58", width: "6", height: "15", rx: "3", fill: "#e8b48f" }
                }
            }

            // legend with live amounts
            rect { x: "40", y: "114", width: "8", height: "6", rx: "1", fill: "#f8fafc", stroke: "#d1d5db", stroke_width: "1" }
            text { x: "52", y: "119", font_size: "8", fill: "#4a1505", "salt — {salt}" }
            rect { x: "40", y: "124", width: "8", height: "6", rx: "1", fill: "#b45309" }
            text { x: "52", y: "129", font_size: "8", fill: "#4a1505", "spice blend — {blend}" }
        }
    }
}

/// Step 6 — bagging the spiced slices and curing them in the fridge.
#[component]
fn CureArt() -> Element {
    rsx! {
        ArtFrame {
            // open zip-lock bag (translucent), being filled
            rect { x: "44", y: "52", width: "78", height: "68", rx: "9", fill: "#e8f1f8",
                stroke: "#93b4cb", stroke_width: "2" }
            // spice flecks on the bag floor / between slices
            for (i , (x , y)) in [
                (66.0_f64, 92.0_f64),
                (98.0, 86.0),
                (72.0, 108.0),
                (100.0, 104.0),
                (84.0, 98.0),
            ]
                .into_iter()
                .enumerate()
            {
                circle { key: "fleck{i}", cx: "{x}", cy: "{y}", r: "1.5", fill: "#b45309" }
            }
            // slices entering from outside through the opening and stacking up: the first,
            // then three more — each descends from above the mouth onto the pile, then they
            // all clear together and the cycle restarts (per-slice keyframe, no delay)
            for (i , cy) in [110.0_f64, 99.0, 88.0, 77.0].into_iter().enumerate() {
                g {
                    key: "stack{i}",
                    class: "anim-stack{i + 1}",
                    style: "--from: {24.0 - cy}px",
                    MeatStrip { cx: 83.0, cy, hl: 27.0, ht: 4.5 }
                }
            }
            // open mouth: parted flaps, rim, zip teeth + slider (in front, so slices pass in)
            line { x1: "45", y1: "52", x2: "39", y2: "44", stroke: "#93b4cb", stroke_width: "2", stroke_linecap: "round" }
            line { x1: "121", y1: "52", x2: "127", y2: "44", stroke: "#93b4cb", stroke_width: "2", stroke_linecap: "round" }
            ellipse { cx: "83", cy: "52", rx: "39", ry: "6", fill: "#cfe0ef", stroke: "#93b4cb", stroke_width: "2" }
            ellipse { cx: "83", cy: "52", rx: "39", ry: "6", fill: "none", stroke: "#7aa3c0", stroke_width: "2", stroke_dasharray: "2 2" }
            rect { x: "118", y: "49", width: "8", height: "6", rx: "1.5", fill: "#7aa3c0" }
            // cure time — compact clock + caption on the right
            circle { cx: "160", cy: "58", r: "22", fill: "#fdf6f0", stroke: "#4a1505", stroke_width: "3" }
            circle { cx: "160", cy: "58", r: "2.5", fill: "#4a1505" }
            g { class: "anim-tick", style: "transform-box: view-box; transform-origin: 160px 58px;",
                line { x1: "160", y1: "58", x2: "160", y2: "41", stroke: "#7c2d12", stroke_width: "3",
                    stroke_linecap: "round" }
            }
            line { x1: "160", y1: "58", x2: "173", y2: "58", stroke: "#a85a32", stroke_width: "2",
                stroke_linecap: "round" }
            text { x: "160", y: "96", text_anchor: "middle", font_size: "11", font_weight: "bold",
                fill: "#4a1505", "24–36 hrs" }
            text { x: "160", y: "107", text_anchor: "middle", font_size: "7", fill: "#7c2d12",
                "flip every 12 h" }
        }
    }
}

/// Step 7 — strips hanging and drying with airflow, plus the target weather conditions.
#[component]
fn DryArt() -> Element {
    let input = calc_input();
    let temp = drying_temp(input.system);
    rsx! {
        ArtFrame {
            // --- target drying conditions ---
            // sun + temperature
            circle { cx: "13", cy: "11", r: "3.5", fill: "#f59e0b" }
            for (i , (x1 , y1 , x2 , y2)) in [
                (13.0_f64, 3.0_f64, 13.0_f64, 6.0_f64),
                (13.0, 16.0, 13.0, 19.0),
                (5.0, 11.0, 8.0, 11.0),
                (18.0, 11.0, 21.0, 11.0),
            ]
                .into_iter()
                .enumerate()
            {
                line {
                    key: "ray{i}",
                    x1: "{x1}",
                    y1: "{y1}",
                    x2: "{x2}",
                    y2: "{y2}",
                    stroke: "#f59e0b",
                    stroke_width: "1.5",
                }
            }
            text { x: "24", y: "14", font_size: "8", font_weight: "bold", fill: "#4a1505", "{temp}" }
            // water drop + humidity
            path { d: "M112 5 q4 5 0 10 q-4 -5 0 -10 z", fill: "#3b82f6" }
            text { x: "120", y: "14", font_size: "8", font_weight: "bold", fill: "#4a1505",
                "50–60% humidity" }

            // hanging rail
            line { x1: "20", y1: "24", x2: "180", y2: "24", stroke: "currentColor", stroke_width: "4",
                stroke_linecap: "round" }
            // swaying biltong strips on hooks (irregular, dried; animated, staggered)
            for (i , x) in [66.0_f64, 100.0, 134.0].into_iter().enumerate() {
                g { key: "{i}", class: "anim-sway",
                    style: "transform-origin: {x}px 24px; animation-delay: {i as f64 * 0.4}s",
                    // S-hook
                    path { d: "M{x} 24 q-5 3 -2 8 q2 3 0 6", fill: "none", stroke: "#b9bcc2", stroke_width: "2" }
                    // dried strip body, knobbly and tapered
                    path {
                        d: "M{x - 8.0} 40 C{x - 10.0} 54 {x - 9.0} 70 {x - 6.0} 84 C{x - 4.0} 92 {x - 2.0} 97 {x} 98 C{x + 2.0} 97 {x + 4.0} 92 {x + 6.0} 84 C{x + 9.0} 70 {x + 10.0} 54 {x + 8.0} 40 C{x + 4.0} 35 {x - 4.0} 35 {x - 8.0} 40 Z",
                        fill: "#54200f",
                    }
                    path { d: "M{x - 2.0} 44 C{x - 4.0} 58 {x - 3.0} 74 {x - 1.0} 86", fill: "none",
                        stroke: "#6e2c18", stroke_width: "3", stroke_linecap: "round", opacity: "0.85" }
                    path { d: "M{x + 3.0} 50 C{x + 4.0} 62 {x + 4.0} 74 {x + 2.0} 84", fill: "none",
                        stroke: "#8a3a26", stroke_width: "1.6", stroke_linecap: "round", opacity: "0.6" }
                }
            }
            // fan airflow lines (animated, staggered)
            for (i , y) in [56.0_f64, 74.0, 92.0].into_iter().enumerate() {
                path { key: "f{i}", class: "anim-airflow",
                    d: "M18 {y} q10 -6 20 0",
                    fill: "none", stroke: "#4d7c0f", stroke_width: "2", stroke_linecap: "round",
                    style: "animation-delay: {i as f64 * 0.3}s" }
            }
            // drying time: a little calendar + "~5 days"
            line { x1: "80", y1: "110", x2: "80", y2: "116", stroke: "currentColor", stroke_width: "1.5" }
            line { x1: "86", y1: "110", x2: "86", y2: "116", stroke: "currentColor", stroke_width: "1.5" }
            rect { x: "76", y: "114", width: "14", height: "12", rx: "2", fill: "#fdf6f0", stroke: "#4a1505", stroke_width: "1.5" }
            rect { x: "76", y: "114", width: "14", height: "4", rx: "2", fill: "#4a1505" }
            text { x: "94", y: "124", font_size: "9", font_weight: "bold", fill: "#4a1505", "~5 days" }
        }
    }
}

/// Step 8 — the finished biltong: whole sticks on a board, sliced and fanned out so the
/// dark rim and red interior of each slice show.
#[component]
fn DoneArt() -> Element {
    rsx! {
        ArtFrame {
            // wooden serving board
            ellipse { cx: "100", cy: "116", rx: "86", ry: "15", fill: "#b07f4f" }
            ellipse { cx: "100", cy: "112", rx: "86", ry: "15", fill: "#caa074" }
            // a fan of biltong slices: dark dried rim, deep-red marbled interior with fat
            // flecks, pivoting from below so they spread out; staggered fan-in
            for i in 0..7usize {
                g { key: "slice{i}", class: "anim-fan-in",
                    style: "animation-delay: {i as f64 * 0.1}s",
                    g { transform: "rotate({(i as f64 - 3.0) * 17.0} 100 128)",
                        ellipse { cx: "100", cy: "90", rx: "12", ry: "19", fill: "#3d1709" }
                        ellipse { cx: "100", cy: "90", rx: "9.6", ry: "16.6", fill: "#7e2a1d" }
                        ellipse { cx: "100", cy: "88", rx: "7", ry: "12", fill: "#9a3b2a", opacity: "0.7" }
                        ellipse { cx: "98", cy: "83", rx: "1.6", ry: "1.1", fill: "#e9d6b4", opacity: "0.85" }
                        ellipse { cx: "103", cy: "89", rx: "1.5", ry: "1", fill: "#e9d6b4", opacity: "0.85" }
                        ellipse { cx: "98", cy: "96", rx: "1.4", ry: "1", fill: "#e9d6b4", opacity: "0.85" }
                    }
                }
            }
        }
    }
}
