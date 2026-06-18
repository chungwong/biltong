//! Slim sticky navigation bar that stays pinned to the top while scrolling, with jump
//! links to the page's three main sections.

use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "sticky top-0 z-50 bg-biltong-900/95 backdrop-blur text-biltong-50 shadow-sm",
            div { class: "max-w-5xl mx-auto px-4 sm:px-6 h-14 flex items-center justify-between gap-3",
                a {
                    href: "#top",
                    class: "font-display font-bold text-lg whitespace-nowrap hover:text-white transition-colors",
                    "Biltong"
                }
                div { class: "flex items-center gap-1 sm:gap-2 text-sm font-semibold",
                    NavLink { href: "#calculator", label: "Calculator" }
                    NavLink { href: "#cuts", label: "Cuts" }
                    NavLink { href: "#steps", label: "Steps" }
                    ThemeToggle {}
                }
            }
        }
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> Element {
    rsx! {
        a {
            href,
            class: "px-2.5 py-1.5 rounded-md text-biltong-100 hover:bg-biltong-700 hover:text-white transition-colors",
            "{label}"
        }
    }
}

/// Light/dark theme switcher. Toggles the `.dark` class on <html> and persists the choice
/// to localStorage; the icon reflects the current theme (read from the DOM on mount, which
/// the no-flash script in index.html has already set from storage/system preference).
#[component]
fn ThemeToggle() -> Element {
    let mut dark = use_signal(|| false);
    use_future(move || async move {
        let mut eval =
            document::eval("dioxus.send(document.documentElement.classList.contains('dark'))");
        if let Ok(v) = eval.recv::<bool>().await {
            dark.set(v);
        }
    });

    let toggle = move |_| {
        let next = !dark();
        dark.set(next);
        let js = if next {
            "document.documentElement.classList.add('dark'); try { localStorage.setItem('biltong:theme','dark'); } catch (e) {}"
        } else {
            "document.documentElement.classList.remove('dark'); try { localStorage.setItem('biltong:theme','light'); } catch (e) {}"
        };
        document::eval(js);
    };

    rsx! {
        button {
            r#type: "button",
            "aria-label": "Toggle dark mode",
            class: "ml-1 p-1.5 rounded-md text-biltong-100 hover:bg-biltong-700 hover:text-white transition-colors",
            onclick: toggle,
            if dark() {
                // sun — click to switch to light
                svg {
                    width: "18", height: "18", view_box: "0 0 24 24", fill: "none",
                    stroke: "currentColor", stroke_width: "2", stroke_linecap: "round",
                    circle { cx: "12", cy: "12", r: "4" }
                    for (i , (x1 , y1 , x2 , y2)) in [
                        (12.0_f64, 1.0_f64, 12.0_f64, 3.0_f64),
                        (12.0, 21.0, 12.0, 23.0),
                        (4.2, 4.2, 5.6, 5.6),
                        (18.4, 18.4, 19.8, 19.8),
                        (1.0, 12.0, 3.0, 12.0),
                        (21.0, 12.0, 23.0, 12.0),
                        (4.2, 19.8, 5.6, 18.4),
                        (18.4, 5.6, 19.8, 4.2),
                    ]
                        .into_iter()
                        .enumerate()
                    {
                        line { key: "ray{i}", x1: "{x1}", y1: "{y1}", x2: "{x2}", y2: "{y2}" }
                    }
                }
            } else {
                // moon — click to switch to dark
                svg {
                    width: "18", height: "18", view_box: "0 0 24 24", fill: "currentColor",
                    path { d: "M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z" }
                }
            }
        }
    }
}
