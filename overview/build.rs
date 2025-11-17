use std::fs;
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    let dest_path = Path::new("./").join(".tailwind");        
    fs::write(
        &dest_path,
        leptos_components::include_generated::all(),
    )?;
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.toml");
    Ok(())
}
