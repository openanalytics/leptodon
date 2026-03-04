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
use proc_macro::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::Expr;
use syn::Lit;
use syn::{Attribute, FnArg, ItemFn, Meta, MetaNameValue, Pat, parse_macro_input};

use crate::util::trim_surrounding_quotes;

/// Extracts doc attribute from parameter attributes
fn extract_doc_attr(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if let Meta::NameValue(MetaNameValue {
            path,
            eq_token: _eq_token,
            value,
        }) = &attr.meta
            && path.is_ident("doc")
            && let Expr::Lit(literal) = value
        {
            let mut tokens = proc_macro2::TokenStream::new();
            literal.to_tokens(&mut tokens);

            let mut doc_literal_str = tokens.to_string();
            if let Lit::Str(_) = literal.lit {
                doc_literal_str = trim_surrounding_quotes(doc_literal_str);
            }
            return Some(doc_literal_str);
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
                param_docs.push((name, "/".to_string()))
            }
        } else {
            param_docs.push(("mauw".to_string(), format!("{arg:?}")))
        }
    }

    // Generate documentation-printing function
    let mut print_body = Vec::new();
    for (i, (name, doc)) in param_docs.iter().enumerate() {
        let classes = if i.is_multiple_of(2) {
            "border-b border-gray-300 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700".to_string()
        } else {
            "border-b border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800".to_string()
        };
        print_body.push(quote! {
            <tr class=#classes>
                <td class="px-5 py-2"><b>{#name}</b></td>
                <td class="px-5 py-2">{#doc}</td>
            </tr>
        });
    }

    let docs_ident = format_ident!("{}Docs", input.sig.ident);
    let table_name_ident = format!("{} Parameters", input.sig.ident);
    let print_fn = quote! {
        /// Produces argument documentation extracted from the related component
        #[component]
        pub fn #docs_ident() -> impl IntoView {
            use leptos::prelude::ClassAttribute;
            use leptos::prelude::ElementChild;

            leptos::prelude::view! {
                <div class="py-2">
                    <crate::heading::Heading5 class="mb-2 mt-2">#table_name_ident</crate::heading::Heading5>
                    <table class="table table-striped table-bordered">
                        <thead>
                            <tr class="text-xs text-gray-700 uppercase bg-gray-200 dark:bg-gray-700 dark:text-gray-300">
                                <th class="px-5 py-2 text-left">"Parameter"</th>
                                <th class="px-5 py-2 text-left">"Documentation"</th>
                            </tr>
                        </thead>
                        <tbody>
                            #(#print_body)*
                        </tbody>
                    </table>
                </div>
            }
        }
    };

    quote! {
        #input
        #print_fn
    }
    .into()
}
