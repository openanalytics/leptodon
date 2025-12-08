use leptos::prelude::ElementChild;
use leptos::prelude::{IntoAny, IntoOptionGetter, MaybeProp};
use leptos::{IntoView, component, prelude::ClassAttribute, view};

use crate::class_list;

#[component]
pub fn Avatar(
    #[prop(optional, into)] src: MaybeProp<String>,
    #[prop(optional, into)] extra_classes: MaybeProp<String>,
) -> impl IntoView {
    if let Some(src) = src.into_option_getter().run() {
        view! {
            <img class=class_list!["w-10 h-10 rounded-full", extra_classes] src=src alt="Rounded avatar"/>
        }.into_any()
    } else {
        view! {
            <div class="relative w-10 h-10 overflow-hidden bg-neutral-secondary-medium rounded-full">
                <svg class="absolute w-12 h-12 text-body-subtle -left-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path></svg>
            </div>
        }.into_any()
    }
}
