use std::fs;
use std::io::Error;
use std::path::Path;

use syn::Visibility;

fn main() -> Result<(), Error> {
    // Unavailable due to https://github.com/leptos-rs/leptos/issues/3813
    let dest_path = Path::new("../").join(".tailwind");
    fs::write(&dest_path, leptos_components::include_generated::all())?;
    Ok(())
}
