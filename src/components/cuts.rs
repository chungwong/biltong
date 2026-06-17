//! Interactive beef-cuts diagram, styled after the MLA "Australian Beef Cuts" chart: a
//! vertical side-of-beef with the hindquarter (round/rump) at the top and the forequarter
//! (chuck/brisket) at the bottom. Tap (mobile) or click (mouse) a cut to see whether it
//! suits biltong; the lean cuts good for biltong are highlighted green. Layout emulated,
//! not copied.

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
    /// Label centre and rotation (degrees; non-zero for the thin tenderloin strip).
    lx: f64,
    ly: f64,
    rot: f64,
}

// Silverside is first so it is the default selection (this recipe's bottom round).
const CUTS: &[Cut] = &[
    Cut {
        label: "Silverside",
        name: "Silverside (bottom round)",
        good: true,
        note: "Lean and even-grained — a top biltong cut. This recipe's bottom round comes \
               from here.",
        points: "120,24 152,24 172,50 104,50",
        lx: 138.0,
        ly: 40.0,
        rot: 0.0,
    },
    Cut {
        label: "Topside",
        name: "Topside (top round)",
        good: true,
        note: "Lean and even — excellent for biltong; this recipe's top round.",
        points: "104,50 138,50 139,112 94,112",
        lx: 116.0,
        ly: 84.0,
        rot: 0.0,
    },
    Cut {
        label: "Knuckle",
        name: "Knuckle (round)",
        good: true,
        note: "A lean hindquarter cut — great for biltong.",
        points: "138,50 172,50 180,74 184,112 139,112",
        lx: 161.0,
        ly: 90.0,
        rot: 0.0,
    },
    Cut {
        label: "Rump",
        name: "Rump",
        good: true,
        note: "Lean and full-flavoured with a good grain — a biltong favourite.",
        points: "94,112 184,112 180,150 96,150",
        lx: 139.0,
        ly: 133.0,
        rot: 0.0,
    },
    Cut {
        label: "Striploin",
        name: "Striploin / short loin",
        good: true,
        note: "Very lean and tender; premium-priced but superb biltong.",
        points: "96,150 130,150 130,228 92,228 86,190",
        lx: 110.0,
        ly: 193.0,
        rot: 0.0,
    },
    Cut {
        label: "Tenderloin",
        name: "Tenderloin (eye fillet)",
        good: true,
        note: "Ultra-lean and tender — pricey, but makes lovely biltong.",
        points: "130,150 148,150 148,228 130,228",
        lx: 139.0,
        ly: 189.0,
        rot: -90.0,
    },
    Cut {
        label: "Flank",
        name: "Flank",
        good: false,
        note: "Lean but very coarse-grained; can turn out chewy.",
        points: "148,150 180,150 186,190 178,228 148,228",
        lx: 165.0,
        ly: 193.0,
        rot: 0.0,
    },
    Cut {
        label: "Cube roll",
        name: "Cube roll (ribeye)",
        good: false,
        note: "Well-marbled and fatty — too rich for good biltong.",
        points: "92,228 178,228 182,290 84,290",
        lx: 132.0,
        ly: 260.0,
        rot: 0.0,
    },
    Cut {
        label: "Chuck",
        name: "Chuck (shoulder)",
        good: false,
        note: "Marbled and sinewy — better braised than dried.",
        points: "84,290 182,290 160,358 120,358",
        lx: 128.0,
        ly: 326.0,
        rot: 0.0,
    },
    Cut {
        label: "Brisket",
        name: "Brisket",
        good: false,
        note: "Fatty and coarse-grained; better cured or braised than dried.",
        points: "180,255 206,260 206,296 180,298",
        lx: 192.0,
        ly: 279.0,
        rot: 0.0,
    },
    Cut {
        label: "Shin",
        name: "Hind shank (shin)",
        good: false,
        note: "Tough and gelatinous — wonderful slow-braised (osso buco), but not for biltong.",
        points: "173,52 215,50 214,74 181,80",
        lx: 198.0,
        ly: 66.0,
        rot: 0.0,
    },
    Cut {
        label: "Shin",
        name: "Fore shank (shin)",
        good: false,
        note: "Tough and gelatinous — wonderful slow-braised, but not for biltong.",
        points: "175,316 214,322 206,348 165,344",
        lx: 197.0,
        ly: 334.0,
        rot: 0.0,
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
                    class: "w-full max-w-xs mx-auto select-none",
                    view_box: "0 0 240 386",
                    role: "img",
                    "aria-label": "Interactive beef cuts diagram",
                    for (i , c) in CUTS.iter().enumerate() {
                        g {
                            key: "cut{i}",
                            class: "cursor-pointer",
                            onclick: move |_| selected.set(i),
                            polygon {
                                points: "{c.points}",
                                fill: cut_fill(c.good, i == sel),
                                stroke: if i == sel { "#2f1a0a" } else { "#ffffff" },
                                stroke_width: if i == sel { "2.5" } else { "1.2" },
                            }
                            text {
                                x: "{c.lx}",
                                y: "{c.ly}",
                                text_anchor: "middle",
                                font_size: "7",
                                font_weight: "bold",
                                fill: "#2a1a0a",
                                pointer_events: "none",
                                transform: "rotate({c.rot} {c.lx} {c.ly})",
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
