//! UI components, each a reusable Dioxus `#[component]`, composed by the root `App`.

mod calculator;
mod cuts;
mod footer;
mod hero;
mod navbar;
mod steps;

pub use calculator::Calculator;
pub use cuts::CutDiagram;
pub use footer::Footer;
pub use hero::Hero;
pub use navbar::NavBar;
pub use steps::Steps;
