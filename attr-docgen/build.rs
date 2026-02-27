// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use walkdir::WalkDir;

// Dumps all source code into OUT_DIR/generated.rs
fn main() -> Result<(), Error> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let mut all_src = String::new();
    for entry in WalkDir::new("src") {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                // println!(
                //     "cargo::warning=path: {} is_file: {}, is_rust: {}",
                //     path.display(),
                //     path.is_file(),
                //     path.ends_with("rs")
                // );
                if path.is_file() {
                    let contents = fs::read_to_string(path)?;
                    all_src += contents.as_str();
                }
            }
            Err(err) => println!("cargo::error={}", err),
        }
    }
    println!("cargo::warning={:?}", &all_src.len());
    println!("cargo::rerun-if-changed=src/");
    println!("cargo::rerun-if-changed=build.rs");
    // Replace every " by \" so it can be embedded in a string
    // all_src = all_src.replace("\"", "\\\"");

    fs::write(
        &dest_path,
        format!(
            "pub fn all_token() -> proc_macro::TokenStream {{
    quote::quote! {{
        r#####\"{all_src}\"#####
    }}.into()
        }}
        "
        ),
    )?;

    Ok(())
}
