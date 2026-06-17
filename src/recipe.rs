//! Single source of truth for the biltong recipe.
//!
//! Everything the page teaches and calculates lives here:
//!   * [`STEPS`]        — the ordered tutorial steps shown on the page.
//!   * [`INGREDIENTS`]  — the spice/cure ratios the calculator scales by meat weight.
//!
//! NOTE: the numbers below are clearly-marked *placeholder* ratios drawn from common
//! biltong practice. Replace the values in [`INGREDIENTS`] (and tweak the step copy if
//! needed) with the exact figures from the source recipe — this is the only place that
//! needs editing.

/// Which inline-SVG animation illustrates a given step.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimKind {
    Slice,
    Vinegar,
    Spice,
    Cure,
    Dry,
    Done,
}

/// One tutorial step.
#[derive(PartialEq)]
pub struct Step {
    pub title: &'static str,
    pub body: &'static str,
    pub anim: AnimKind,
}

/// The ordered steps for making biltong.
pub const STEPS: &[Step] = &[
    Step {
        title: "Choose & slice the beef",
        body: "Pick a lean cut such as silverside or topside. Trim off sinew and slice \
               with the grain into strips about 1–2 cm thick — thinner dries faster, \
               thicker stays chewy in the middle.",
        anim: AnimKind::Slice,
    },
    Step {
        title: "Vinegar wash",
        body: "Splash the strips with brown, malt or apple-cider vinegar and toss to coat. \
               The acid seasons the meat, helps it hold spice, and protects the surface \
               while it dries.",
        anim: AnimKind::Vinegar,
    },
    Step {
        title: "Mix & apply the spice cure",
        body: "Toast the coriander seed until fragrant, then crack it coarse. Combine with \
               salt, black pepper, sugar and (for safe slow drying) a little Cure #1. Rub \
               the mix evenly over every strip.",
        anim: AnimKind::Spice,
    },
    Step {
        title: "Cure & rest",
        body: "Pack the spiced strips into a container and rest in the fridge for 12–24 \
               hours, turning once or twice. This lets the salt and flavour work their way \
               into the meat.",
        anim: AnimKind::Cure,
    },
    Step {
        title: "Hang & dry",
        body: "Hang the strips with good airflow in a biltong box at roughly 25 °C and low \
               humidity. Give them 3–5 days — longer for a drier, harder result.",
        anim: AnimKind::Dry,
    },
    Step {
        title: "Slice & enjoy",
        body: "It's ready when the outside is firm but the strip still has a little spring. \
               Slice across the grain into thin pieces and enjoy. Store in a paper bag so \
               it can keep breathing.",
        anim: AnimKind::Done,
    },
];

/// How an ingredient scales with the amount of meat.
#[derive(Clone, Copy)]
pub enum Measure {
    /// Grams of ingredient per gram of meat (a fraction, e.g. `0.02` = 2 % of meat weight).
    WeightFraction(f64),
    /// Millilitres of ingredient per kilogram of meat.
    VolumePerKg(f64),
}

/// One ingredient in the spice cure.
pub struct Ingredient {
    pub name: &'static str,
    pub measure: Measure,
    pub note: &'static str,
}

/// Spice/cure ratios. **Placeholder values — replace with the source recipe's figures.**
pub const INGREDIENTS: &[Ingredient] = &[
    Ingredient {
        name: "Coarse salt",
        measure: Measure::WeightFraction(0.020),
        note: "~2% of meat weight",
    },
    Ingredient {
        name: "Coriander seed (toasted, cracked)",
        measure: Measure::WeightFraction(0.015),
        note: "the signature biltong spice",
    },
    Ingredient {
        name: "Black pepper (coarse)",
        measure: Measure::WeightFraction(0.005),
        note: "",
    },
    Ingredient {
        name: "Brown sugar",
        measure: Measure::WeightFraction(0.010),
        note: "balances the salt",
    },
    Ingredient {
        name: "Cure #1 (pink curing salt)",
        measure: Measure::WeightFraction(0.0025),
        note: "optional, for safe slow drying",
    },
    Ingredient {
        name: "Vinegar (brown / cider)",
        measure: Measure::VolumePerKg(40.0),
        note: "for the wash",
    },
    Ingredient {
        name: "Worcestershire sauce",
        measure: Measure::VolumePerKg(10.0),
        note: "optional, for depth",
    },
];
