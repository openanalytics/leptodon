use std::fs;
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    // Unavailable due to https://github.com/leptos-rs/leptos/issues/3813
    let dest_path = Path::new("../").join(".tailwind");
    fs::write(&dest_path, leptodon::include_generated::all())?;
    Ok(())
}
