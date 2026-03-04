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
