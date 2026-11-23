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

mod codeblock;
mod include_generated;
mod parameter_docs;
mod svg;
mod util;

#[proc_macro_attribute]
pub fn generate_codeblock(attr: TokenStream, item: TokenStream) -> TokenStream {
    codeblock::_generate_codeblock(attr, item)
}

#[proc_macro_attribute]
pub fn generate_docs(attr: TokenStream, item: TokenStream) -> TokenStream {
    parameter_docs::_generate_docs(attr, item)
}

#[proc_macro]
pub fn generate_all_source(_: TokenStream) -> TokenStream {
    include_generated::all_token()
}

/// Parses its argument as RSTML markup and expands to a `&'static str` literal
/// containing the rendered SVG.
///
/// The markup is validated by the parser at compile time, so malformed SVG is a
/// hard compile error. Because the result is baked in as a string literal,
/// nothing is built at runtime.
///
/// # Example
/// ```
/// use leptodon_proc_macros::svg;
/// let icon: &'static str = svg! {
///     <path fill-rule="evenodd" d="M11 3.29Z" clip-rule="evenodd" />
/// };
/// assert_eq!(icon, r#"<path fill-rule="evenodd" d="M11 3.29Z" clip-rule="evenodd"/>"#);
/// ```
///
/// Multiple top-level nodes and nesting are supported:
/// ```
/// use leptodon_proc_macros::svg;
/// let icon: &'static str = svg! {
///     <g stroke-width="2">
///         <path d="M4 4h16" />
///         <rect x="1" y="1" width="3" height="3" />
///     </g>
/// };
/// assert_eq!(icon, r#"<g stroke-width="2"><path d="M4 4h16"/><rect x="1" y="1" width="3" height="3"/></g>"#);
/// ```
///
/// # Errors
///
/// Malformed markup is a compile error, e.g. an unclosed element:
/// ```compile_fail
/// use leptodon_proc_macros::svg;
/// let _ = svg! {
///     <g stroke-width="2">
///         <path d="M4 4h16" />
/// };
/// ```
///
/// Dynamic expressions cannot be baked into a `&'static str`, so they are also
/// rejected at compile time:
/// ```compile_fail
/// use leptodon_proc_macros::svg;
/// let d = "M0 0";
/// let _ = svg! { <path d={d} /> };
/// ```
#[proc_macro]
pub fn svg(tokens: TokenStream) -> TokenStream {
    svg::_svg(tokens)
}
