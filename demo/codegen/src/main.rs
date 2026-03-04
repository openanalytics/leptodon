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
use std::io::Write;
use std::path::Path;

fn attr_all() -> &'static str {
    leptodon_proc_macros::generate_all_source!()
}

fn main() -> Result<(), Error> {
    // buil.rs is unavailable due to https://github.com/leptos-rs/leptos/issues/3813
    let mut dest = std::fs::File::create(Path::new("../").join(".tailwind"))?;
    write!(dest, "{}", leptodon::include_generated::all())?;
    write!(dest, "{}", attr_all())?;
    Ok(())
}
