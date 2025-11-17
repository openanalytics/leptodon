General CSS Architecture:
 - Leptos-components exposes its source code as a string via `leptos_components::include_generated::message()`
 - `build.rs` takes that source code and puts it in a `.tailwind` file
 - Leptos-components and therefore this project uses tailwind to do styling, tailwind works by scanning the code for tokens that it deems possible class-names.
   Only these discovered class names are then generated in a stylesheet.
 - Running tailwind's css generation is a manual process and is done via tailwind's npm package, to install: `npm install`, to run the css generation: `npx tailwindcss -i input.css -o style/output.css --watch`
 - Current we use tailwind v3, tailwind will look at its tailwind.config.js and therein it'll find other files to scan. Some of these directories are the `src/**.rs` files and `.tailwind`
 - files and directories for css:
   - .tailwind
   - style/output.css
   - input.css
   - package.json
   - package-lock.json
   - postcss.config.js
   - tailwind.config.js
   - node_modules
   
Leptos client side rendering with Trunk:
  - This overview demo expects to be ran with Trunk
  - Trunk will produce all the web resource: html, css, wasm based on the source.
  - Configurable via `Trunk.toml`
  - Looks for trunk properties on html elements e.g. `<link data-trunk rel="css" href="style/output.css" />`
  - Support live updating on rust source changes with `trunk serve`, add `--open` to open the dev-server's url in your browser automatically.
  - Can install the trunk cli with `cargo install trunk --locked`, make sure `.cargo/bin` is in your `$PATH` https://trunkrs.dev/#install
  - Expects the `wasm32-unknown-unknown` to be installed `rustup target add wasm32-unknown-unknown`
  