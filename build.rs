use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> Result<(), Error> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let mut all_src = String::new();
    for entry in WalkDir::new("src") {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                println!(
                    "cargo::warning={}",
                    format!(
                        "path: {} is_file: {}, is_rust: {}",
                        path.display(),
                        path.is_file(),
                        path.ends_with("rs")
                    )
                );
                if path.is_file() {
                    let contents = fs::read_to_string(path)?;
                    all_src += contents.as_str();
                }
            }
            Err(err) => println!("cargo::error={}", err),
        }
    }
    println!("cargo::warning={}", format!("{:?}", &all_src.len()));
    // Replace every " by \" so it can be embedded in a string
    // all_src = all_src.replace("\"", "\\\"");
    fs::write(
        &dest_path,
        format!(
            "pub fn all() -> &'static str {{
            r#\"{all_src}\"#
        }}
        "
        ),
    )?;

    Ok(())
}
