//! Interactive beef-cuts diagram. Tap (mobile) or click (mouse) a cut on the cow to see
//! whether it suits biltong; the lean hindquarter cuts are highlighted green.
//!
//! The cow is an MLA-style side profile (emulated, not copied). Cut regions tile the body
//! as straight-edged polygons; the fore/hind shanks are drawn as symmetric legs and are
//! themselves selectable "Shin" cuts.

use dioxus::prelude::*;

struct Cut {
    /// Short label drawn on the diagram.
    label: &'static str,
    /// Full name shown in the info panel.
    name: &'static str,
    /// Whether the cut is well suited to biltong.
    good: bool,
    note: &'static str,
    /// Polygon points for the region (SVG user units).
    points: &'static str,
    /// Label centre.
    lx: f64,
    ly: f64,
}

// Round is first so it is the default selection (the cut this recipe uses). Body cuts come
// before the shanks so the shank legs draw on top of the haunch/brisket.
const CUTS: &[Cut] = &[
    Cut {
        label: "Round",
        name: "Round — topside, silverside & eye",
        good: true,
        note: "Lean, even-grained and the classic biltong cut. This recipe's bottom/top \
               round comes from here.",
        points: "250,100 300,100 312,116 300,134 250,133",
        lx: 276.0,
        ly: 118.0,
    },
    Cut {
        label: "Rump",
        name: "Rump",
        good: true,
        note: "Lean and full-flavoured with a good grain — a biltong favourite.",
        points: "250,60 300,60 300,100 250,100",
        lx: 275.0,
        ly: 82.0,
    },
    Cut {
        label: "Loin",
        name: "Striploin / short loin (incl. fillet)",
        good: true,
        note: "Very lean and tender; makes excellent — if premium — biltong.",
        points: "200,62 250,60 250,100 200,100",
        lx: 225.0,
        ly: 83.0,
    },
    Cut {
        label: "Rib",
        name: "Rib",
        good: false,
        note: "Well-marbled and fatty — too rich for good biltong.",
        points: "150,66 200,62 200,100 150,100",
        lx: 175.0,
        ly: 84.0,
    },
    Cut {
        label: "Chuck",
        name: "Chuck (shoulder)",
        good: false,
        note: "Flavourful but marbled and sinewy — better braised than dried.",
        points: "95,72 150,66 150,100 95,100",
        lx: 121.0,
        ly: 87.0,
    },
    Cut {
        label: "Brisket",
        name: "Brisket",
        good: false,
        note: "Fatty and coarse-grained; better cured or braised than dried.",
        points: "95,100 150,100 150,130 95,126",
        lx: 121.0,
        ly: 116.0,
    },
    Cut {
        label: "Flank",
        name: "Flank",
        good: false,
        note: "Lean but very coarse-grained; can turn out chewy.",
        points: "150,100 250,100 250,133 150,130",
        lx: 200.0,
        ly: 117.0,
    },
    Cut {
        label: "Shin",
        name: "Fore shank (shin)",
        good: false,
        note: "Tough and full of connective tissue — wonderful slow-braised (osso buco), \
               but not for biltong.",
        points: "109,124 127,124 126,158 124,192 112,192 110,158",
        lx: 118.0,
        ly: 150.0,
    },
    Cut {
        label: "Shin",
        name: "Hind shank (shin)",
        good: false,
        note: "Tough and full of connective tissue — wonderful slow-braised, but not for \
               biltong.",
        points: "287,128 305,128 304,158 302,192 290,192 288,158",
        lx: 296.0,
        ly: 152.0,
    },
];

#[component]
pub fn CutDiagram() -> Element {
    let mut selected = use_signal(|| 0usize);
    let sel = selected();
    let cut = &CUTS[sel];

    rsx! {
        section { id: "cuts", class: "max-w-4xl mx-auto px-6 py-16",
            h2 { class: "font-display text-3xl font-bold text-biltong-700 text-center mb-3",
                "Which cut of beef?"
            }
            p { class: "text-center text-stone-600 mb-8 max-w-xl mx-auto",
                "Lean cuts from the hindquarter make the best biltong. Tap a cut on the cow \
                 to see whether it works."
            }

            div { class: "grid md:grid-cols-2 gap-8 items-center",
                // --- diagram ---
                svg {
                    class: "w-full select-none",
                    view_box: "0 0 360 210",
                    role: "img",
                    "aria-label": "Interactive beef cuts diagram",

                    // tail (behind the body)
                    path {
                        d: "M300 62 q24 6 20 42 q-2 12 -11 15",
                        fill: "none",
                        stroke: "#a98a6f",
                        stroke_width: "3",
                        stroke_linecap: "round",
                    }
                    circle { cx: "309", cy: "121", r: "4", fill: "#4a1505" }

                    // far legs (decorative, behind the body) — same shape as the near legs
                    polygon { points: "129,124 147,124 146,156 144,188 132,188 130,156", fill: "#bfa389" }
                    polygon { points: "267,130 285,130 284,156 282,188 270,188 268,156", fill: "#bfa389" }

                    // cut regions (clickable); shanks last so they sit on top of the body
                    for (i , c) in CUTS.iter().enumerate() {
                        g {
                            key: "cut{i}",
                            class: "cursor-pointer",
                            onclick: move |_| selected.set(i),
                            polygon {
                                points: "{c.points}",
                                fill: cut_fill(c.good, i == sel),
                                stroke: if i == sel { "#2f1a0a" } else { "#ffffff" },
                                stroke_width: if i == sel { "2.5" } else { "1" },
                            }
                            text {
                                x: "{c.lx}",
                                y: "{c.ly}",
                                text_anchor: "middle",
                                font_size: "7.5",
                                font_weight: "bold",
                                fill: "#2a1a0a",
                                pointer_events: "none",
                                "{c.label}"
                            }
                        }
                    }

                    // hooves (decorative)
                    rect { x: "110", y: "189", width: "17", height: "6", rx: "1", fill: "#4a1505" }
                    rect { x: "288", y: "189", width: "18", height: "6", rx: "1", fill: "#4a1505" }

                    // head & neck (decorative, on top)
                    polygon { points: "95,74 74,66 52,72 40,88 44,104 64,112 95,118",
                        fill: "#c9ad93", stroke: "#a98a6f", stroke_width: "1" }
                    polygon { points: "60,66 68,50 75,66", fill: "#c9ad93", stroke: "#a98a6f", stroke_width: "1" }
                    ellipse { cx: "42", cy: "94", rx: "4", ry: "6", fill: "#b08d72" }
                    circle { cx: "56", cy: "82", r: "2.5", fill: "#2a1a0a" }
                }

                // --- info panel ---
                div { class: "bg-white rounded-2xl shadow-sm ring-1 ring-biltong-100 p-6",
                    h3 { class: "font-display text-2xl font-semibold text-biltong-900 mb-3",
                        "{cut.name}"
                    }
                    if cut.good {
                        span { class: "inline-block px-3 py-1 rounded-full text-sm font-semibold bg-herb-600 text-white",
                            "✓ Great for biltong"
                        }
                    } else {
                        span { class: "inline-block px-3 py-1 rounded-full text-sm font-semibold bg-stone-200 text-stone-600",
                            "Not ideal"
                        }
                    }
                    p { class: "text-stone-600 leading-relaxed mt-3", "{cut.note}" }
                }
            }

            // legend
            div { class: "flex items-center justify-center gap-6 mt-8 text-sm text-stone-600",
                span { class: "flex items-center gap-2",
                    span { class: "inline-block w-4 h-4 rounded", style: "background:#9caf88" }
                    "Good for biltong"
                }
                span { class: "flex items-center gap-2",
                    span { class: "inline-block w-4 h-4 rounded", style: "background:#d8a47f" }
                    "Other cuts"
                }
            }
        }
    }
}

/// Region fill colour from (suitable-for-biltong, currently-selected).
fn cut_fill(good: bool, selected: bool) -> &'static str {
    match (good, selected) {
        (true, true) => "#6f8f5a",
        (true, false) => "#9caf88",
        (false, true) => "#b06a3e",
        (false, false) => "#d8a47f",
    }
}
