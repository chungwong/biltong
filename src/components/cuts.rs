//! Interactive beef-cuts diagram, styled after the MLA "Australian Beef Cuts" chart: a
//! horizontal side of beef with the forequarter (chuck/blade/brisket) on the left and the
//! hindquarter (round/rump) on the right. Tap (mobile) or click (mouse) a cut to see
//! whether it suits biltong; the lean cuts good for biltong are highlighted green. Layout
//! emulated, not copied.

use dioxus::prelude::*;

/// How well a cut suits biltong. `Divisive` covers cuts (like brisket) that some makers
/// swear by and others avoid.
#[derive(Clone, Copy, PartialEq)]
enum Rating {
    Good,
    Divisive,
    Poor,
}

struct Cut {
    /// Short label drawn on the diagram.
    label: &'static str,
    /// Full name shown in the info panel.
    name: &'static str,
    /// How well the cut suits biltong.
    rating: Rating,
    note: &'static str,
    /// Polygon points for the region (SVG user units).
    points: &'static str,
    /// Label centre.
    lx: f64,
    ly: f64,
}

// Silverside is first so it is the default selection (this recipe's bottom round).
const CUTS: &[Cut] = &[
    Cut {
        label: "Silverside",
        name: "Silverside (bottom round)",
        rating: Rating::Good,
        note: "Lean and even-grained — a top biltong cut. This recipe's bottom round comes \
               from here.",
        points: "288,90 356,96 354,123 288,122",
        lx: 320.0,
        ly: 110.0,
    },
    Cut {
        label: "Topside",
        name: "Topside (top round)",
        rating: Rating::Good,
        note: "Lean and even — excellent for biltong; this recipe's top round.",
        points: "288,56 338,60 356,96 288,90",
        lx: 316.0,
        ly: 80.0,
    },
    Cut {
        label: "Knuckle",
        name: "Knuckle (round)",
        rating: Rating::Good,
        note: "A lean hindquarter cut — great for biltong.",
        points: "288,122 354,123 349,152 300,152",
        lx: 318.0,
        ly: 140.0,
    },
    Cut {
        label: "Rump",
        name: "Rump",
        rating: Rating::Good,
        note: "Lean and full-flavoured with a good grain — a biltong favourite.",
        points: "235,54 288,56 288,105 235,105",
        lx: 261.0,
        ly: 84.0,
    },
    Cut {
        label: "Striploin",
        name: "Striploin / short loin",
        rating: Rating::Good,
        note: "Very lean and tender; premium-priced but superb biltong.",
        points: "185,55 235,54 235,105 185,105",
        lx: 210.0,
        ly: 84.0,
    },
    Cut {
        label: "Tenderloin",
        name: "Tenderloin (eye fillet)",
        rating: Rating::Good,
        note: "Ultra-lean and tender — pricey, but makes lovely biltong.",
        points: "185,105 288,105 288,120 185,120",
        lx: 236.0,
        ly: 116.0,
    },
    Cut {
        label: "Flank",
        name: "Flank",
        rating: Rating::Good,
        note: "Lean and tasty biltong — just slice with the grain, as it is coarse.",
        points: "140,105 185,105 185,120 288,120 288,156 140,156",
        lx: 215.0,
        ly: 140.0,
    },
    Cut {
        label: "Cube roll",
        name: "Cube roll (ribeye)",
        rating: Rating::Poor,
        note: "Well-marbled and fatty — too rich for good biltong.",
        points: "135,58 185,55 185,105 135,105",
        lx: 160.0,
        ly: 84.0,
    },
    Cut {
        label: "Blade",
        name: "Blade (shoulder)",
        rating: Rating::Poor,
        note: "Sinewy, with a seam of connective tissue — better braised than dried.",
        points: "92,62 135,58 135,105 92,105",
        lx: 113.0,
        ly: 86.0,
    },
    Cut {
        label: "Chuck",
        name: "Chuck (neck/shoulder)",
        rating: Rating::Poor,
        note: "Marbled and sinewy — better braised than dried.",
        points: "44,74 92,62 92,105 50,105",
        lx: 71.0,
        ly: 88.0,
    },
    Cut {
        label: "Brisket",
        name: "Brisket",
        rating: Rating::Divisive,
        note: "Divisive: some makers love a well-trimmed brisket biltong, others find it \
               too fatty. Trim hard and slice thin if you try it.",
        points: "50,105 140,105 140,156 50,156",
        lx: 92.0,
        ly: 132.0,
    },
    Cut {
        label: "Shin",
        name: "Fore shank (shin)",
        rating: Rating::Poor,
        note: "Tough and gelatinous — wonderful slow-braised (osso buco), but not for biltong.",
        points: "72,156 96,156 92,206 78,206",
        lx: 84.0,
        ly: 184.0,
    },
    Cut {
        label: "Shin",
        name: "Hind shank (shin)",
        rating: Rating::Poor,
        note: "Tough and gelatinous — wonderful slow-braised, but not for biltong.",
        points: "300,152 322,152 318,206 305,206",
        lx: 311.0,
        ly: 184.0,
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
                "Lean cuts from the hindquarter make the best biltong. Tap a cut to see \
                 whether it works."
            }

            div { class: "grid md:grid-cols-2 gap-8 items-center",
                // --- diagram ---
                svg {
                    class: "w-full select-none",
                    view_box: "0 0 380 225",
                    role: "img",
                    "aria-label": "Interactive beef cuts diagram",
                    for (i , c) in CUTS.iter().enumerate() {
                        g {
                            key: "cut{i}",
                            class: "cursor-pointer",
                            onclick: move |_| selected.set(i),
                            polygon {
                                points: "{c.points}",
                                fill: cut_fill(c.rating, i == sel),
                                stroke: if i == sel { "#2f1a0a" } else { "#ffffff" },
                                stroke_width: if i == sel { "2.5" } else { "1.3" },
                            }
                            text {
                                x: "{c.lx}",
                                y: "{c.ly}",
                                text_anchor: "middle",
                                font_size: "6.6",
                                font_weight: "bold",
                                fill: "#2a1a0a",
                                pointer_events: "none",
                                "{c.label}"
                            }
                        }
                    }
                    // decorative hooves on the two shank legs
                    rect { x: "76", y: "202", width: "18", height: "6", rx: "1", fill: "#4a1505" }
                    rect { x: "303", y: "202", width: "18", height: "6", rx: "1", fill: "#4a1505" }
                }

                // --- info panel ---
                div { class: "bg-white rounded-2xl shadow-sm ring-1 ring-biltong-100 p-6",
                    h3 { class: "font-display text-2xl font-semibold text-biltong-900 mb-3",
                        "{cut.name}"
                    }
                    match cut.rating {
                        Rating::Good => rsx! {
                            span { class: "inline-block px-3 py-1 rounded-full text-sm font-semibold bg-herb-600 text-white",
                                "✓ Great for biltong"
                            }
                        },
                        Rating::Divisive => rsx! {
                            span { class: "inline-block px-3 py-1 rounded-full text-sm font-semibold bg-amber-500 text-white",
                                "↔ Divisive — some swear by it"
                            }
                        },
                        Rating::Poor => rsx! {
                            span { class: "inline-block px-3 py-1 rounded-full text-sm font-semibold bg-stone-200 text-stone-600",
                                "Not ideal"
                            }
                        },
                    }
                    p { class: "text-stone-600 leading-relaxed mt-3", "{cut.note}" }
                }
            }

            // legend
            div { class: "flex flex-wrap items-center justify-center gap-x-6 gap-y-2 mt-8 text-sm text-stone-600",
                span { class: "flex items-center gap-2",
                    span { class: "inline-block w-4 h-4 rounded", style: "background:#9caf88" }
                    "Good for biltong"
                }
                span { class: "flex items-center gap-2",
                    span { class: "inline-block w-4 h-4 rounded", style: "background:#e3b34e" }
                    "Worth a try (divisive)"
                }
                span { class: "flex items-center gap-2",
                    span { class: "inline-block w-4 h-4 rounded", style: "background:#d8a47f" }
                    "Other cuts"
                }
            }
        }
    }
}

/// Region fill colour from (biltong rating, currently-selected).
fn cut_fill(rating: Rating, selected: bool) -> &'static str {
    match (rating, selected) {
        (Rating::Good, true) => "#6f8f5a",
        (Rating::Good, false) => "#9caf88",
        (Rating::Divisive, true) => "#c2922f",
        (Rating::Divisive, false) => "#e3b34e",
        (Rating::Poor, true) => "#b06a3e",
        (Rating::Poor, false) => "#d8a47f",
    }
}
