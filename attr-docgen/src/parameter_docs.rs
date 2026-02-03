use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Expr;
use syn::{Attribute, FnArg, ItemFn, Meta, MetaNameValue, Pat, parse_macro_input};

/// Extracts doc attribute from parameter attributes
fn extract_doc_attr(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if let Meta::NameValue(MetaNameValue {
            path,
            eq_token: _eq_token,
            value,
        }) = &attr.meta
        {
            if path.is_ident("doc") {
                if let Expr::Lit(s) = value {
                    return Some(format!("{:?}", &s.lit));
                }
            }
        }
    }
    None
}
#[proc_macro_attribute]
pub fn generate_docs(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    // input.attrs = Vec::new(); // Remove macro attribute

    // CORRECTED: PROPERLY POPULATE param_docs USING extract_doc_attr
    let mut param_docs = Vec::new();
    param_docs.push(("sanity".to_string(), "check".to_string()));
    for arg in &input.sig.inputs {
        if let FnArg::Typed(pat) = arg {
            let name = match &*pat.pat {
                Pat::Ident(ident) => ident.ident.to_string(),
                _ => continue,
            };
            let doc = extract_doc_attr(&pat.attrs);
            if let Some(doc_str) = doc {
                param_docs.push((name, doc_str));
            } else {
                param_docs.push(("mauw".to_string(), format!("no docs found on {name}")))
            }
        } else {
            param_docs.push(("mauw".to_string(), format!("{arg:?}")))
        }
    }
    
    // Generate documentation-printing function
    let mut print_body = Vec::new();
    print_body.push(quote! {
        <table class="table table-striped table-bordered">
            <thead>
                <tr>
                    <th>Parameter</th>
                    <th>Documentation</th>
                </tr>
            </thead>
            <tbody>
    });
    for (name, doc) in &param_docs {
        print_body.push(quote! {
            <tr>
                <td>{#name}</td>
                <td>{#doc}</td>
            </tr>
        });
    }
    print_body.push(quote! {
            </tbody>
        </table>
    });

    let docs_ident = format_ident!("{}Docs", input.sig.ident);
    let print_fn = quote! {
        /// Produces argument documentation extracted from the related component
        #[component]
        pub fn #docs_ident() -> impl IntoView {
            leptos::prelude::view! {
                #(#print_body)*
            }
        }
    };

    quote! {
        #input
        #print_fn
    }
    .into()
}
