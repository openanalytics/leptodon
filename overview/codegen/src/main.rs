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

use syn::Visibility;

fn gen_icons() -> Result<(), Error> {
    let icons = Path::new("../src").join("gen_icons.rs");
    let leptodon_icons = syn::parse_file(
        std::fs::read_to_string("../../src/icon/mod.rs")
            .expect("Icon source must be present.")
            .as_str(),
    )
    .expect("valid source code");

    let func_count = leptodon_icons.items.len();
    let functions = leptodon_icons.items.iter().filter(|item| {
        if let syn::Item::Fn(item_fn) = item {
            item_fn.sig.inputs.is_empty()
        } else {
            false
        }
    });

    let icon_html = functions
        .map(|func| match func {
            syn::Item::Fn(item_fn) if matches!(item_fn.vis, Visibility::Public(_)) => {
                format!(
                    r#"
            <span>
                <Icon icon=leptodon::icon::{}() class="border-2 border-solid w-24 h-24"/>
                {}
            </span>
"#,
                    item_fn.sig.ident,
                    item_fn.sig.ident
                )
            }
            _ => String::new(),
        })
        .collect::<Vec<_>>()
        .join("");
    fs::write(
        icons,
        format!(
            r#"use leptos::prelude::ElementChild;
use leptos::{{IntoView, component, view}};
use leptodon::icon::Icon;

#[component]
pub fn IconList() -> impl IntoView {{
    view! {{{}    }}
}}

// {}
"#,
            icon_html, func_count
        ),
    )?;
    Ok(())
}

fn main() -> Result<(), Error> {
    // Unavailable due to https://github.com/leptos-rs/leptos/issues/3813
    let dest_path = Path::new("../").join(".tailwind");
    fs::write(&dest_path, leptodon::include_generated::all())?;
    gen_icons()?;
    Ok(())
}
