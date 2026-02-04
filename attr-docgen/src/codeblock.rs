use quote::ToTokens;
use proc_macro::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

fn wrong_attrs() -> ! {
    panic!("Wrong attribute format, expected generate_codeblock(Component, ComponentExample)");
}
pub(crate) fn _generate_codeblock(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    // let mut component_name = String::new();
    let mut example_name = String::new();
    for (i, token) in attr.into_iter().enumerate() {
        match i {
            0 => {
                let TokenTree::Ident(identifier) = token else {
                    wrong_attrs();
                };
                example_name = identifier.span().source_text().unwrap();
            }
            _ => {
                panic!("Too many arguments for generate_codeblock, expected 2 identifiers.");
            }
        }
    }
        
    let mut body_tokens = proc_macro2::TokenStream::new();
    input.block.to_tokens(&mut body_tokens);
    let body = body_tokens
       .into_iter()
       .filter_map(|t| t.span().source_text())
       .reduce(|p, n| p + n.as_str())
       .unwrap(); 
    let demo_ident = &input.sig.ident;
    let example_ident = format_ident!("{}", example_name);
    let codeblock_ident = format_ident!("{}Codeblock", &demo_ident);

    quote! {
        #input

        #[component]
        pub fn #codeblock_ident() -> impl IntoView {
            leptos::prelude::view!{
                <leptos_components::codeblock::Codeblock code=#body />
            }
        }
        
        #[component]
        pub fn #example_ident() -> impl IntoView {
            leptos::prelude::view! {
                <leptos_components::form_input::Label required=false label="Example Toggle usage">
                    <div class="flex flex-col border-1 border border-black rounded-lg shadow-sm w-fit p-4 min-w-[50vw]">
                        <div class="p-3">
                            <#demo_ident />
                        </div>
                        <hr class="mb-4"></hr>
                        <#codeblock_ident />
                    </div>
                </leptos_components::form_input::Label>
            }
        }
    }
    .into()
}
