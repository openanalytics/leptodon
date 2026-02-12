> [!NOTE]  
> Codegen being in its own crate works around build.rs building leptos with the wrong cfg flags. 
> And then trying to reuse that wrongly built dependency for the real binary causing linking errors.

## Demo codgen
 - Generates `.tailwind` containing all the leptos-components source code for tailwind's class generation.