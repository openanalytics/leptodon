use proc_macro::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::Expr;
use syn::Lit;
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
                if let Expr::Lit(literal) = value {
                    let mut tokens = proc_macro2::TokenStream::new();
                    literal.to_tokens(&mut tokens);

                    let mut doc_literal_str = tokens.to_string();
                    if let Lit::Str(_) = literal.lit
                        && doc_literal_str.ends_with("\"")
                        && doc_literal_str.starts_with("\"")
                    {
                        // Assumes string literal is surrounded by " ";
                        doc_literal_str.truncate(doc_literal_str.len() - 1);
                        doc_literal_str = doc_literal_str[1..].trim().to_string();
                    }
                    return Some(doc_literal_str);
                }
            }
        }
    }
    None
}

pub(crate) fn _generate_docs(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    // input.attrs = Vec::new(); // Remove macro attribute

    let mut param_docs = Vec::new();
    // param_docs.push(("sanity".to_string(), "check".to_string()));
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
                param_docs.push((name, format!("/")))
            }
        } else {
            param_docs.push(("mauw".to_string(), format!("{arg:?}")))
        }
    }

    // Generate documentation-printing function
    let mut print_body = Vec::new();
    for (name, doc) in &param_docs {
        print_body.push(quote! {
            <tr>
                <td>{#name}</td>
                <td>{#doc}</td>
            </tr>
        });
    }

    let docs_ident = format_ident!("{}Docs", input.sig.ident);
    let print_fn = quote! {
        /// Produces argument documentation extracted from the related component
        #[component]
        pub fn #docs_ident() -> impl IntoView {
            leptos::prelude::view! {
                <table class="table table-striped table-bordered">
                    <thead>
                        <tr>
                            <th>"Parameter"</th>
                            <th>"Documentation"</th>
                        </tr>
                    </thead>
                    <tbody>
                        #(#print_body)*
                    </tbody>
                </table>
            }
        }
    };

    quote! {
        #input
        #print_fn
    }
    .into()
}
