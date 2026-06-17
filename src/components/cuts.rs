//! Interactive beef-cuts diagram. Tap (mobile) or click (mouse) a cut on the cow to see
//! whether it suits biltong; the lean hindquarter cuts are highlighted green.

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

// Round is first so it is the default selection (the cut this recipe uses).
const CUTS: &[Cut] = &[
    Cut {
        label: "Round",
        name: "Round — topside, silverside & eye",
        good: true,
        note: "Lean, even-grained and the classic biltong cut. This recipe's bottom/top \
               round comes from here.",
        points: "236,80 288,80 288,128 236,128",
        lx: 262.0,
        ly: 106.0,
    },
    Cut {
        label: "Rump",
        name: "Rump",
        good: true,
        note: "Lean and full-flavoured with a good grain — a biltong favourite.",
        points: "236,52 288,52 288,80 236,80",
        lx: 262.0,
        ly: 68.0,
    },
    Cut {
        label: "Sirloin",
        name: "Sirloin",
        good: true,
        note: "Lean and tender; makes excellent biltong.",
        points: "198,52 236,52 236,90 198,90",
        lx: 217.0,
        ly: 73.0,
    },
    Cut {
        label: "Loin",
        name: "Short loin — striploin & fillet",
        good: true,
        note: "Very lean but premium-priced; lovely tender biltong if you splurge.",
        points: "158,52 198,52 198,90 158,90",
        lx: 178.0,
        ly: 73.0,
    },
    Cut {
        label: "Chuck",
        name: "Chuck (shoulder)",
        good: false,
        note: "Flavourful but marbled and sinewy — better braised than dried.",
        points: "72,52 120,52 120,90 72,90",
        lx: 96.0,
        ly: 73.0,
    },
    Cut {
        label: "Rib",
        name: "Rib",
        good: false,
        note: "Well-marbled and fatty — too rich for good biltong.",
        points: "120,52 158,52 158,90 120,90",
        lx: 139.0,
        ly: 73.0,
    },
    Cut {
        label: "Brisket",
        name: "Brisket",
        good: false,
        note: "Fatty and coarse-grained; better cured or braised than dried.",
        points: "72,90 120,90 120,128 72,128",
        lx: 96.0,
        ly: 111.0,
    },
    Cut {
        label: "Plate",
        name: "Plate",
        good: false,
        note: "Fatty belly cut — not suited to biltong.",
        points: "120,90 158,90 158,128 120,128",
        lx: 139.0,
        ly: 111.0,
    },
    Cut {
        label: "Flank",
        name: "Flank",
        good: false,
        note: "Lean but very coarse-grained; can turn out chewy.",
        points: "158,90 236,90 236,128 158,128",
        lx: 197.0,
        ly: 111.0,
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
                    view_box: "0 0 320 200",
                    role: "img",
                    "aria-label": "Interactive beef cuts diagram",
                    // tail
                    path {
                        d: "M288 56 q16 4 13 32 q-1 9 -8 13",
                        fill: "none",
                        stroke: "#a98a6f",
                        stroke_width: "3",
                        stroke_linecap: "round",
                    }
                    circle { cx: "294", cy: "103", r: "3.5", fill: "#4a1505" }
                    // legs
                    for (i , lx) in [84.0_f64, 108.0, 248.0, 274.0].into_iter().enumerate() {
                        g { key: "leg{i}",
                            rect { x: "{lx}", y: "124", width: "9", height: "46", rx: "2",
                                fill: "#c9ad93", stroke: "#a98a6f", stroke_width: "1" }
                            rect { x: "{lx}", y: "166", width: "9", height: "5", rx: "1", fill: "#4a1505" }
                        }
                    }
                    // head
                    polygon { points: "72,60 50,58 34,70 34,96 50,106 72,102",
                        fill: "#c9ad93", stroke: "#a98a6f", stroke_width: "1" }
                    polygon { points: "58,58 65,45 71,58", fill: "#c9ad93", stroke: "#a98a6f", stroke_width: "1" }
                    ellipse { cx: "35", cy: "92", rx: "4", ry: "6", fill: "#b08d72" }
                    circle { cx: "47", cy: "74", r: "2.5", fill: "#2a1a0a" }

                    // cut regions (clickable)
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
