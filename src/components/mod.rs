//! UI components, each a reusable Dioxus `#[component]`, composed by the root `App`.

mod calculator;
mod footer;
mod hero;
mod steps;

pub use calculator::Calculator;
pub use footer::Footer;
pub use hero::Hero;
pub use steps::Steps;
