use std::io::Error;
use std::process::Command;

use std::path::Path;
use std::fs;

use heck::ToTitleCase;
use syn::Visibility;
use walkdir::WalkDir;

pub struct PageInfo {
    pub location: String,
    pub display_name: String,
    pub page_comp_ident: String,
}

fn find_page_details(path: &Path) -> PageInfo {
    let page_source = syn::parse_file(
        std::fs::read_to_string(path)
            .expect("Source of [path] must be present to find page details.")
            .as_str(),
    )
    .expect("syn parseable source code");

    let mut functions = page_source.items.iter().filter(|item| {
        if let syn::Item::Fn(item_fn) = item {
            item_fn.sig.inputs.len() == 0
        } else {
            false
        }
    });

    let page_info = functions
        .find_map(|func| match func {
            syn::Item::Fn(item_fn)
                if matches!(item_fn.vis, Visibility::Public(_))
                    && item_fn.sig.ident.to_string().ends_with("Page") =>
            {
                let location = path
                    .file_stem()
                    .expect("file should have filename")
                    .display();
                let display_name = item_fn
                    .sig
                    .ident
                    .to_string()
                    .trim_end_matches("Page")
                    .trim_end_matches("Demo")
                    .to_title_case();
                let page_comp_ident = item_fn.sig.ident.to_string();
                Some(PageInfo {
                    location: format!("{location}"),
                    display_name,
                    page_comp_ident: format!("crate::demos::{location}::{page_comp_ident}"),
                })
            }
            _ => None,
        })
        .expect(format!("{} should have a ...Page() function", path.display()).as_str());
    page_info
}

fn generate_demo_pages() -> Result<(), Error> {
    let dest_path = Path::new("src").join("generated_demolist.rs");
    let mut page_infos = vec![];

    // Walk through demo pages
    for entry in WalkDir::new("src/demos") {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    let file_name = path
                        .file_name()
                        .expect("Files should be named")
                        .display()
                        .to_string();
                    if file_name == "mod.rs" {
                        continue;
                    }

                    // Extra demo page function info, save it to page_infos
                    page_infos.push(find_page_details(path));
                }
            }
            Err(err) => println!("cargo::error={}", err),
        }
    }
    println!("cargo::warning={:?} demos.", &page_infos.len());

    let page_info_entries = page_infos
        .iter()
        .map(|page_info| {
            let location = &page_info.location;
            let display_name = &page_info.display_name;
            format!(
                r#"
        PageInfo {{
            location: "/demo/{location}",
            name: "{display_name}"
        }}"#
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    
    let page_routes = page_infos
        .iter()
        .map(|page_info| {
            let location = &page_info.location;
            let page_comp_ident = &page_info.page_comp_ident;
            format!(
                r#"            <Route path=path!("/{location}") view={page_comp_ident}/>"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Construct demo-list file
    fs::write(
        &dest_path,
        format!(
            "
use leptos::{{component, view}};
use leptos_router::components::{{Outlet, ParentRoute, Route}};
use leptos_router::path;
use leptos_router::any_nested_route::IntoAnyNestedRoute;

pub struct PageInfo {{
    pub location: &'static str,
    pub name: &'static str
}}

pub fn page_infos() -> Vec<PageInfo> {{
    vec![
{page_info_entries}
    ]
}}

#[component(transparent)]
pub fn DemoRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {{
    view! {{
        <ParentRoute path=path!(\"demo\") view=Outlet>
{page_routes}
        </ParentRoute>
    }}
    .into_inner()
    .into_any_nested_route()
}}
"
        ),
    )?;

    Ok(())
}

fn main() -> Result<(), Error> {
    println!("cargo:rustc-env=RUSTFLAGS=--cfg=erase_components");
    // See ./codegen/README.md
    Command::new("cargo")
        .current_dir("codegen")
        .args(["run"])
        .output()
        .expect("failed to execute process");

    // page_infos() and DemoRoutes
    generate_demo_pages()?;
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/demos");
    println!("cargo::rerun-if-changed=Cargo.toml");
    Ok(())
}
