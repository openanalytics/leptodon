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
use std::io::Error;
use std::process::Command;

fn main() -> Result<(), Error> {
    println!("cargo:rustc-env=RUSTFLAGS=--cfg=erase_components");
    // See ./codegen/README.md
    Command::new("cargo")
        .current_dir("codegen")
        .args(["run"])
        .output()
        .expect("failed to execute process");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.toml");
    Ok(())
}
