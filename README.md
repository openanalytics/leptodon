<h1 align="center"><img alt="Leptodon Logo" width="50%" src="./assets/banner.svg"></h1>
<h3 align="center">Your <a href="https://github.com/leptos-rs">Leptos</a> UI toolkit.</h3>
<p align="center">
    <a href="https://crates.io/crates/leptodon"><img alt="Latest Version" src="https://img.shields.io/crates/v/leptodon"></a>
    <a href="https://github.com/openanalytics/leptodon/blob/main/LICENSE"><img alt="GitHub License" src="https://img.shields.io/crates/l/leptodon"></a>
    <a href="https://docs.rs/leptodon/latest/leptodon/"><img alt="GitHub Release" src="https://img.shields.io/docsrs/leptodon"></a>
</p>

<p align="center">
    <a href="#getting-started">Getting Started</a> | <a href="https://leptodon.dev">Documentation</a> | <a href="https://github.com/openanalytics/leptodon-starter">Template</a>
</p>

## Getting started
1. Clone the [template repository](https://github.com/openanalytics/leptodon-starter).
2. Pick and place components from [the documentation](https://leptodon.dev).

## Contributing
1. Please enable the pre-commit hook for code styling.
2. Use `cargo make mimic-ci` to test locally.

### Development dependencies
A couple tools are required to work on this project:
- `cargo-make` for the [Makefile.toml](./Makefile.toml).
- `cargo-leptos` for serving and building the application.
- `cargo-nextest` for running the unit tests.
- `npm` and `playwright` for running the end-to-end tests (there is a nix devenv in [overview/end2end](./overview/end2end)).
- `licensure` for license header checks.

### Project layout
- [demo](./demo): leptos-ssr demo application, serves as developer documentation where they can preview demonstrations of Leptodon's components.
  - [demo/codegen](./demo/codegen): Generates a .tailwind file (containing all Leptodon's source code for Tailwind to generate CSS against).
- [proc-macros](./proc-macros): Code generation for the demo, see the [demo README](./demo/README.md) for more information on how this is used.
- [overview](./overview): leptos-ssr testing application, a suite of [playwright](https://playwright.dev/) tests is ran against this application, also serves as development zone for new components.
  - [overview/end2end](./overview/end2end): The aforementioned playwright tests.
  - [overview/codegen](./overview/codegen): Generates both the .tailwind file (containing all Leptodon's source code for Tailwind to generate CSS against) as well as an IconList component.


## Credits
- [ThawUI](https://thawui.vercel.app/): component architecture inspiration + utilities [Copyright (c) 2023 lizidev]
- [Flowbite](https://flowbite.com/docs/getting-started/introduction/): general UI design [Copyright (c) Themesberg (Bergside Inc.)]
