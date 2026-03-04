Leptodon
==============

Your [Leptos](https://github.com/leptos-rs/) UI toolkit.

## Getting started
1. Use the template repository: //TODO: ... fill in
2. Pick and place components from: https://leptodon.dev

## Contributing
1. Please enable the pre-commit hook for code styling.
2. `cargo make mimic-ci` to locally test.

### Dev-dependencies
A couple tools are required to work on this project:
- `cargo-make` for the [Makefile.toml](./Makefile.toml)
- `cargo-leptos` for leptos
- `cargo-nextest` for unit-tests
- `npm` and `playwright` for end2end tests (There is a nix devenv in overview/end2end).
- `licensure` for license header checks

### Project layout
- [demo](./demo): leptos-ssr demo application, serves as developer-docs where they can preview demonstrations of leptodon's components.
  - [overview/codegen](./overview/codegen): Generates a .tailwind file (containing all leptodon's source code for tailwind to generate css against).
- [proc-macros](./proc-macros): Code-generation for the demo, see the [demo README](./demo/README.md) for more info on how this is used.
- [overview](./overview): leptos-ssr testing application, a suite of [playwright](https://playwright.dev/) tests is ran against this application, also serves as dev-zone for new components.
  - [overview/end2end](./overview/end2end): The affordmentioned playwright tests.
  - [overview/codegen](./overview/codegen): Generates both the .tailwind file (containing all leptodon's source code for tailwind to generate css against) as well as an IconList component.


## Credits
- [ThawUI](https://thawui.vercel.app/): component architecture inspiration + utilities [Copyright (c) 2023 lizidev]
- [Flowbite](https://flowbite.com/docs/getting-started/introduction/): general UI design [Copyright (c) Themesberg (Bergside Inc.)]
