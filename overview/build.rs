use std::fs;
use std::io::Error;
use std::path::Path;

fn gen_icons() -> Result<(), Error> {
    let icons = Path::new("./src").join("gen_icons.rs");
    let leptos_components_icons = syn::parse_file(
        std::fs::read_to_string("../src/icon/mod.rs")
            .expect("Icon source must be present.")
            .as_str(),
    )
    .expect("valid source code");

    let func_count = leptos_components_icons.items.iter().count();
    let functions = leptos_components_icons.items.iter().filter(|item| {
        if let syn::Item::Fn(item_fn) = item {
            item_fn.sig.inputs.len() == 0
        } else {
            false
        }
    });

    let icon_html = functions
        .map(|func| match func {
            syn::Item::Fn(item_fn) => {
                format!(
                    r#"
            <p>
                <Icon icon=leptos_components::icon::{}() class="border-2 border-solid w-24 h-24"/>
            </p>
"#,
                    item_fn.sig.ident.to_string()
                )
            }
            _ => todo!(),
        })
        .collect::<Vec<_>>()
        .join("");
    fs::write(
        icons,
        format!(
            r#"
use leptos::prelude::ElementChild;
use leptos::{{IntoView, component, view}};
use leptos_components::icon::Icon;

#[component]
pub fn IconList() -> impl IntoView {{
    view! {{
        {}
    }}
}}

// {}
        "#,
            icon_html, func_count
        ),
    )?;
    Ok(())
}

fn main() -> Result<(), Error> {
    println!("cargo:rustc-env=RUSTFLAGS=--cfg=erase_components");
    // Unavailable due to https://github.com/leptos-rs/leptos/issues/3813
    let dest_path = Path::new("./").join(".tailwind");
    // fs::write(&dest_path, leptos_components::include_generated::all())?;
    gen_icons()?;
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.toml");
    Ok(())
}
