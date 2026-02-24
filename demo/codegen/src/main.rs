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
use std::fs;
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    // Unavailable due to https://github.com/leptos-rs/leptos/issues/3813
    let dest_path = Path::new("../").join(".tailwind");
    fs::write(&dest_path, leptodon::include_generated::all())?;
    Ok(())
}
