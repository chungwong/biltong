//! Pure calculation + unit-conversion logic for the spice calculator.
//!
//! Kept free of any UI so it can be unit-tested. The UI lives in
//! [`crate::components::calculator`] and calls [`compute`].

use crate::recipe::{Ingredient, Measure, INGREDIENTS};

const GRAMS_PER_OZ: f64 = 28.349_523_125;
const GRAMS_PER_LB: f64 = 453.592_37;
const ML_PER_FLOZ: f64 = 29.573_529_562_5;

/// The unit system the user is working in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitSystem {
    /// Meat in kilograms; output weights in grams, volumes in millilitres.
    Metric,
    /// Meat in pounds; output weights in ounces, volumes in fluid ounces.
    Imperial,
}

impl UnitSystem {
    /// Label for the meat-weight input field.
    pub fn meat_unit(self) -> &'static str {
        match self {
            UnitSystem::Metric => "kg",
            UnitSystem::Imperial => "lb",
        }
    }
}

/// One computed row: an ingredient name, its scaled amount (already formatted in the
/// chosen unit), plus the bits the chart needs — a short label, a unit-independent
/// magnitude for bar sizing, and a bar colour.
#[derive(Clone, PartialEq)]
pub struct ResultLine {
    pub name: &'static str,
    pub short: &'static str,
    pub amount: String,
    pub note: &'static str,
    /// Quantity in base metric units (grams for solids, millilitres for liquids). Used
    /// only for relative bar sizing, so it is independent of the chosen display unit.
    pub magnitude: f64,
    /// Bar colour for the chart (spice amber for solids, red for liquids).
    pub color: &'static str,
}

/// Convert a meat-weight input (in the system's meat unit) into grams.
pub fn meat_to_grams(value: f64, system: UnitSystem) -> f64 {
    match system {
        UnitSystem::Metric => value * 1000.0,
        UnitSystem::Imperial => value * GRAMS_PER_LB,
    }
}

fn format_weight(grams: f64, system: UnitSystem) -> String {
    match system {
        UnitSystem::Metric => format!("{} g", round1(grams)),
        UnitSystem::Imperial => format!("{} oz", round1(grams / GRAMS_PER_OZ)),
    }
}

fn format_volume(ml: f64, system: UnitSystem) -> String {
    match system {
        UnitSystem::Metric => format!("{} ml", round1(ml)),
        UnitSystem::Imperial => format!("{} fl oz", round1(ml / ML_PER_FLOZ)),
    }
}

/// Round to one decimal place, dropping a trailing `.0`.
fn round1(value: f64) -> String {
    let rounded = (value * 10.0).round() / 10.0;
    if (rounded.fract()).abs() < f64::EPSILON {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded:.1}")
    }
}

fn line_for(ing: &Ingredient, meat_grams: f64, system: UnitSystem) -> ResultLine {
    let (amount, magnitude, color) = match ing.measure {
        Measure::WeightFraction(fraction) => {
            let grams = meat_grams * fraction;
            (format_weight(grams, system), grams, "#b45309")
        }
        Measure::VolumePerKg(ml_per_kg) => {
            let ml = meat_grams / 1000.0 * ml_per_kg;
            (format_volume(ml, system), ml, "#b91c1c")
        }
    };
    ResultLine {
        name: ing.name,
        short: ing.short,
        amount,
        note: ing.note,
        magnitude,
        color,
    }
}

/// Scale every ingredient to `meat_grams` of meat, formatted for `system`.
pub fn compute(meat_grams: f64, system: UnitSystem) -> Vec<ResultLine> {
    INGREDIENTS
        .iter()
        .map(|ing| line_for(ing, meat_grams, system))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_meat_input_is_kilograms() {
        assert!((meat_to_grams(1.0, UnitSystem::Metric) - 1000.0).abs() < 1e-9);
        assert!((meat_to_grams(2.5, UnitSystem::Metric) - 2500.0).abs() < 1e-9);
    }

    #[test]
    fn imperial_meat_input_is_pounds() {
        assert!((meat_to_grams(1.0, UnitSystem::Imperial) - GRAMS_PER_LB).abs() < 1e-6);
    }

    #[test]
    fn reproduces_source_recipe_at_base_weight() {
        // The source quotes its amounts for 4540 g of meat; the calculator must match.
        use crate::recipe::BASE_MEAT_G;
        let lines = compute(BASE_MEAT_G, UnitSystem::Metric);
        let amount = |name: &str| {
            lines
                .iter()
                .find(|l| l.name == name)
                .unwrap_or_else(|| panic!("missing {name}"))
                .amount
                .clone()
        };
        assert_eq!(amount("Salt"), "102 g");
        assert_eq!(amount("Coriander seed (toasted)"), "68.1 g");
        assert_eq!(amount("Peppercorns"), "34 g");
        assert_eq!(amount("Chili flakes"), "22.7 g");
        assert_eq!(amount("Red wine vinegar"), "120 ml");
        assert_eq!(amount("Worcestershire sauce"), "60 ml");
    }

    #[test]
    fn salt_is_about_2_2_percent_per_kilogram() {
        // Salt is 102 g / 4540 g ~= 2.2%; 1 kg -> ~22.5 g.
        let lines = compute(meat_to_grams(1.0, UnitSystem::Metric), UnitSystem::Metric);
        assert_eq!(lines[0].name, "Salt");
        assert_eq!(lines[0].amount, "22.5 g");
    }

    #[test]
    fn imperial_formats_in_ounces() {
        // 1 lb of meat, salt at ~2.2% -> ~10.2 g -> ~0.4 oz.
        let lines = compute(
            meat_to_grams(1.0, UnitSystem::Imperial),
            UnitSystem::Imperial,
        );
        assert_eq!(lines[0].name, "Salt");
        assert_eq!(lines[0].amount, "0.4 oz");
    }

    #[test]
    fn round1_drops_trailing_zero() {
        assert_eq!(round1(20.0), "20");
        assert_eq!(round1(20.04), "20");
        assert_eq!(round1(20.05), "20.1");
    }
}
