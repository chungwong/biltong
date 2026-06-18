//! Pure calculation + unit-conversion logic for the spice calculator.
//!
//! Kept free of any UI so it can be unit-tested. The UI lives in
//! [`crate::components::calculator`] and calls [`compute`].

use crate::recipe::{Ingredient, Measure, INGREDIENTS};
use std::collections::HashMap;

const GRAMS_PER_OZ: f64 = 28.349_523_125;
const GRAMS_PER_LB: f64 = 453.592_37;
const ML_PER_FLOZ: f64 = 29.573_529_562_5;

/// Per-ingredient override factors, keyed by ingredient name. The factor is in the
/// ingredient's native units (a weight fraction, or millilitres per kg) so it rescales with
/// the meat weight exactly like the recipe defaults.
pub type Overrides = HashMap<String, f64>;

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

/// Shared calculator input: the meat weight (always stored in grams) and the chosen unit
/// system. Provided as context so the step animations can show live amounts too.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CalcInput {
    pub meat_grams: f64,
    pub system: UnitSystem,
}

impl Default for CalcInput {
    fn default() -> Self {
        // Matches the calculator's default of "1" in metric (1 kg).
        CalcInput {
            meat_grams: 1000.0,
            system: UnitSystem::Metric,
        }
    }
}

/// One computed row: an ingredient name, the scaled amount as a number plus its unit, an
/// optional note, whether it's a volume (vs weight) ingredient, and whether the amount has
/// been overridden by the user.
#[derive(Clone, PartialEq)]
pub struct ResultLine {
    pub name: &'static str,
    pub value: f64,
    pub unit: &'static str,
    pub note: String,
    pub is_volume: bool,
    pub overridden: bool,
}

impl ResultLine {
    /// The amount formatted as "<value> <unit>", e.g. "22.5 g".
    pub fn amount(&self) -> String {
        format!("{} {}", round1(self.value), self.unit)
    }

    /// The numeric value rounded for display in an editable field.
    pub fn value_str(&self) -> String {
        round1(self.value)
    }
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

/// Round to one decimal place, dropping a trailing `.0`.
pub fn round1(value: f64) -> String {
    let rounded = (value * 10.0).round() / 10.0;
    if (rounded.fract()).abs() < f64::EPSILON {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded:.1}")
    }
}

fn line_for(
    ing: &Ingredient,
    meat_grams: f64,
    system: UnitSystem,
    override_factor: Option<f64>,
) -> ResultLine {
    let (value, unit, is_volume, note) = match ing.measure {
        Measure::WeightFraction(fraction) => {
            let frac = override_factor.unwrap_or(fraction);
            let grams = meat_grams * frac;
            // Salt (and any `percent` ingredient) shows its live ratio, not a fixed string.
            let note = if ing.percent {
                format!("~{}% of meat weight", round1(frac * 100.0))
            } else {
                ing.note.to_string()
            };
            match system {
                UnitSystem::Metric => (grams, "g", false, note),
                UnitSystem::Imperial => (grams / GRAMS_PER_OZ, "oz", false, note),
            }
        }
        Measure::VolumePerKg(ml_per_kg) => {
            let ml = meat_grams / 1000.0 * override_factor.unwrap_or(ml_per_kg);
            match system {
                UnitSystem::Metric => (ml, "ml", true, ing.note.to_string()),
                UnitSystem::Imperial => (ml / ML_PER_FLOZ, "fl oz", true, ing.note.to_string()),
            }
        }
    };
    ResultLine {
        name: ing.name,
        value,
        unit,
        note,
        is_volume,
        overridden: override_factor.is_some(),
    }
}

/// Scale every ingredient to `meat_grams` of meat, formatted for `system`, applying any
/// per-ingredient `overrides`.
pub fn compute(meat_grams: f64, system: UnitSystem, overrides: &Overrides) -> Vec<ResultLine> {
    INGREDIENTS
        .iter()
        .map(|ing| line_for(ing, meat_grams, system, overrides.get(ing.name).copied()))
        .collect()
}

/// Back-calculate an ingredient's override factor from an edited amount (entered in the
/// current unit system), so the edit rescales with the meat weight like the recipe defaults.
pub fn factor_from_amount(value: f64, meat_grams: f64, system: UnitSystem, is_volume: bool) -> f64 {
    if meat_grams <= 0.0 {
        return 0.0;
    }
    if is_volume {
        let ml = match system {
            UnitSystem::Metric => value,
            UnitSystem::Imperial => value * ML_PER_FLOZ,
        };
        ml / (meat_grams / 1000.0)
    } else {
        let grams = match system {
            UnitSystem::Metric => value,
            UnitSystem::Imperial => value * GRAMS_PER_OZ,
        };
        grams / meat_grams
    }
}

/// Slice thickness for the cut step, in the chosen unit system (2 cm ≈ ¾ in).
pub fn slice_thickness(system: UnitSystem) -> &'static str {
    match system {
        UnitSystem::Metric => "2 cm",
        UnitSystem::Imperial => "¾ in",
    }
}

/// Target drying temperature range, in the chosen unit system. Shared by the dry-step
/// text and its animation so they always agree.
pub fn drying_temp(system: UnitSystem) -> &'static str {
    match system {
        UnitSystem::Metric => "21–27 °C",
        UnitSystem::Imperial => "70–80 °F",
    }
}

/// Format the meat weight itself (kg or lb) for display in the slice animation.
pub fn format_meat(meat_grams: f64, system: UnitSystem) -> String {
    match system {
        UnitSystem::Metric => format!("{} kg", round1(meat_grams / 1000.0)),
        UnitSystem::Imperial => format!("{} lb", round1(meat_grams / GRAMS_PER_LB)),
    }
}

/// The formatted amount of a single ingredient by name (empty string if not found),
/// honouring any user override. Used by the step animations so they match the calculator.
pub fn amount_of(name: &str, meat_grams: f64, system: UnitSystem, overrides: &Overrides) -> String {
    INGREDIENTS
        .iter()
        .find(|ing| ing.name == name)
        .map(|ing| line_for(ing, meat_grams, system, overrides.get(name).copied()).amount())
        .unwrap_or_default()
}

/// The combined spice blend (every weight ingredient except salt: coriander, pepper, chili),
/// honouring any user overrides.
pub fn spice_blend_amount(meat_grams: f64, system: UnitSystem, overrides: &Overrides) -> String {
    let grams: f64 = INGREDIENTS
        .iter()
        .filter_map(|ing| match ing.measure {
            Measure::WeightFraction(fraction) if ing.name != "Salt" => {
                Some(meat_grams * overrides.get(ing.name).copied().unwrap_or(fraction))
            }
            _ => None,
        })
        .sum();
    format_weight(grams, system)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn amounts(meat_grams: f64, system: UnitSystem) -> Vec<ResultLine> {
        compute(meat_grams, system, &Overrides::new())
    }

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
        let lines = amounts(BASE_MEAT_G, UnitSystem::Metric);
        let amount = |name: &str| {
            lines
                .iter()
                .find(|l| l.name == name)
                .unwrap_or_else(|| panic!("missing {name}"))
                .amount()
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
        let lines = amounts(meat_to_grams(1.0, UnitSystem::Metric), UnitSystem::Metric);
        assert_eq!(lines[0].name, "Salt");
        assert_eq!(lines[0].amount(), "22.5 g");
    }

    #[test]
    fn imperial_formats_in_ounces() {
        // 1 lb of meat, salt at ~2.2% -> ~10.2 g -> ~0.4 oz.
        let lines = amounts(
            meat_to_grams(1.0, UnitSystem::Imperial),
            UnitSystem::Imperial,
        );
        assert_eq!(lines[0].name, "Salt");
        assert_eq!(lines[0].amount(), "0.4 oz");
    }

    #[test]
    fn round1_drops_trailing_zero() {
        assert_eq!(round1(20.0), "20");
        assert_eq!(round1(20.04), "20");
        assert_eq!(round1(20.05), "20.1");
    }

    #[test]
    fn an_override_rescales_with_the_meat_weight() {
        // Edit salt to 30 g at 1 kg -> a 3% fraction that scales to 60 g at 2 kg.
        let factor = factor_from_amount(30.0, 1000.0, UnitSystem::Metric, false);
        let mut ov = Overrides::new();
        ov.insert("Salt".to_string(), factor);
        let at1 = compute(1000.0, UnitSystem::Metric, &ov);
        let at2 = compute(2000.0, UnitSystem::Metric, &ov);
        assert_eq!(at1[0].amount(), "30 g");
        assert_eq!(at2[0].amount(), "60 g");
        assert!(at1[0].overridden);
    }

    #[test]
    fn volume_override_round_trips() {
        // Edit vinegar to 200 ml at 1 kg, then read it back at 1 kg.
        let factor = factor_from_amount(200.0, 1000.0, UnitSystem::Metric, true);
        let mut ov = Overrides::new();
        ov.insert("Red wine vinegar".to_string(), factor);
        let lines = compute(1000.0, UnitSystem::Metric, &ov);
        let vin = lines.iter().find(|l| l.name == "Red wine vinegar").unwrap();
        assert_eq!(vin.amount(), "200 ml");
    }
}
