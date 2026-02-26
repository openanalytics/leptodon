General CSS Architecture:
 - Leptodon exposes its source code as a string via `leptodon::include_generated::message()`
 - `build.rs` takes that source code and puts it in a `.tailwind` file
 - Leptodon and therefore this project uses tailwind to do styling, tailwind works by scanning the code for tokens that it deems possible class-names.
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
