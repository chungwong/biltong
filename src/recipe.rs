//! Single source of truth for the biltong recipe.
//!
//! Everything the page teaches and calculates lives here:
//!   * [`STEPS`]        — the ordered tutorial steps shown on the page.
//!   * [`INGREDIENTS`]  — the spice/cure ratios the calculator scales by meat weight.
//!
//! The ratios in [`INGREDIENTS`] are taken directly from the source recipe, which is
//! built around a [`BASE_MEAT_G`]-gram batch; each amount is stored as `source / batch`
//! so the calculator reproduces the original numbers exactly at that weight and scales
//! linearly from there. This is the only place recipe numbers need editing.

/// Which inline-SVG animation illustrates a given step.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimKind {
    Slice,
    Vinegar,
    Toast,
    Grind,
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
        body: "Use a lean cut like bottom round or top round. Trim off the silver skin, \
               gristle and soft fat, then cut WITH the grain into 2 cm thick steaks — \
               about the width of your thumb.",
        anim: AnimKind::Slice,
    },
    Step {
        title: "Vinegar & Worcestershire bath",
        body: "Mix the red wine vinegar and Worcestershire sauce. Pour half into a tray, \
               lay the steaks on top, then pour the rest over so every piece is coated.",
        anim: AnimKind::Vinegar,
    },
    Step {
        title: "Toast the coriander",
        body: "Toast the coriander seeds in a dry pan over medium heat, shaking often, until \
               they turn fragrant and start to pop — a minute or two. Toasting wakes up the \
               aroma that defines biltong. Tip them out to cool.",
        anim: AnimKind::Toast,
    },
    Step {
        title: "Grind the spices",
        body: "Once cool, grind the coriander and peppercorns (and the chili flakes) to a \
               coarse crack — not a fine powder, so you keep texture and get a good spice \
               crust. Combine everything into one spice blend.",
        anim: AnimKind::Grind,
    },
    Step {
        title: "Salt & spice the meat",
        body: "Rub half the salt and enough spice blend into the steaks to coat them, then \
               flip and work in the rest. Hold a little blend back to dust on at the end for \
               an extra spice crust.",
        anim: AnimKind::Spice,
    },
    Step {
        title: "Bag & cure",
        body: "Pack the meat with all the spices and juices into a vacuum or zip-lock bag \
               and press out the air. Cure in the fridge for 24–36 hours, flipping and \
               massaging the bag every 12 hours so it cures evenly.",
        anim: AnimKind::Cure,
    },
    Step {
        title: "Weigh & hang to dry",
        body: "Weigh and note each steak, hook it, and hang in a warm, sunny spot with a \
               gentle breeze (21–27 °C / 70–80 °F, 50–60% humidity). Dry until it loses \
               about 50% of its weight for 'wet' biltong, 55–60% for medium, or up to 70% \
               for dry.",
        anim: AnimKind::Dry,
    },
    Step {
        title: "Slice & store",
        body: "Once it's dried to your liking, slice across the grain and enjoy. \
               Vacuum-seal any extra and keep it in the fridge or freezer.",
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

/// The meat weight (grams) the source recipe's amounts are quoted for.
pub const BASE_MEAT_G: f64 = 4540.0;

/// Spice/cure ratios, taken from the source recipe (quoted for [`BASE_MEAT_G`] of meat).
pub const INGREDIENTS: &[Ingredient] = &[
    Ingredient {
        name: "Salt",
        measure: Measure::WeightFraction(102.0 / BASE_MEAT_G),
        note: "~2.2% of meat weight",
    },
    Ingredient {
        name: "Coriander seed (toasted)",
        measure: Measure::WeightFraction(68.1 / BASE_MEAT_G),
        note: "the signature biltong spice",
    },
    Ingredient {
        name: "Peppercorns",
        measure: Measure::WeightFraction(34.0 / BASE_MEAT_G),
        note: "coarsely ground",
    },
    Ingredient {
        name: "Chili flakes",
        measure: Measure::WeightFraction(22.7 / BASE_MEAT_G),
        note: "optional, for heat",
    },
    Ingredient {
        name: "Red wine vinegar",
        measure: Measure::VolumePerKg(120.0 / (BASE_MEAT_G / 1000.0)),
        note: "for the bath",
    },
    Ingredient {
        name: "Worcestershire sauce",
        measure: Measure::VolumePerKg(60.0 / (BASE_MEAT_G / 1000.0)),
        note: "for the bath",
    },
];
