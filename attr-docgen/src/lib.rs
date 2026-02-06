use proc_macro::TokenStream;

mod codeblock;
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
