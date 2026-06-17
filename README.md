# Biltong

A small, pure-WebAssembly site that teaches you how to make biltong, with an
ingredient calculator that scales the spice cure to any amount of beef. Built with
[Dioxus](https://dioxuslabs.com) and [Tailwind CSS](https://tailwindcss.com), and hosted
on GitHub Pages — no server, no tracking.

Method and recipe adapted from the excellent guide by
[Two Guys & A Cooler](https://twoguysandacooler.com/biltong/).

## Features

- Step-by-step guide, each step illustrated with a hand-built **inline-SVG animation**
  (CSS keyframes only — no JS, honours `prefers-reduced-motion`).
- **Spice calculator** with a **metric ⇄ imperial** toggle that recomputes reactively.
- Single page, no client-side router, so it works under the GitHub Pages sub-path.

## Project layout

```
tailwind.css            Tailwind v4 input + custom @keyframes for the animations
index.html              page <head> / mount point
src/
  main.rs               app entry; composes the components
  recipe.rs             SINGLE source of truth: steps + ingredient ratios
  calculator.rs         pure calc + unit-conversion logic (unit-tested)
  components/           Hero, Steps, Calculator, Footer
  animations/           one inline-SVG component per step
.github/workflows/      GitHub Pages build + deploy
```

### Editing the recipe

All recipe content lives in [`src/recipe.rs`](src/recipe.rs): `STEPS` (the tutorial copy)
and `INGREDIENTS` (the spice/cure ratios the calculator uses). The ratios there are
clearly-marked placeholders — replace them with your exact figures in that one file.

## Develop

Requires the Rust toolchain, the `wasm32-unknown-unknown` target, and the Dioxus CLI:

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.9 --locked

dx serve --platform web        # live-reload dev server (also compiles Tailwind)
cargo test                     # run the calculator unit tests
```

## Build a production bundle

```sh
dx bundle --platform web --release
```

The static site is written under `target/dx/biltong/release/web/public`.

## Deploy

Pushing to the deploy branch runs `.github/workflows/deploy.yml`, which builds the bundle
and publishes it to GitHub Pages. **One-time setup:** in the repo settings, set
**Pages → Build and deployment → Source** to **GitHub Actions**. The site then serves at
`https://chungwong.github.io/biltong/` (the `base_path` in `Dioxus.toml` matches the repo
name so asset URLs resolve under the sub-path).
