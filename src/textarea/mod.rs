use leptos::prelude::BindAttribute;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::Show;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, RwSignal},
    view,
};

const TA_CLASSES: &'static str = "block p-2.5 w-full resize text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
const LABEL_CLASSES: &'static str = "block mb-2 text-sm font-medium text-gray-900 dark:text-white";
#[component]
pub fn TextArea(
    #[prop(optional, into)] label: MaybeProp<String>,
    input: RwSignal<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <Show
            when=move || {
                !label.get().as_ref().map(String::is_empty).unwrap_or(true)
            }
            fallback=|| ()
        >
            <label for="message" class=LABEL_CLASSES>{move || label.get()}</label>
        </Show>
        <textarea
            id="message"
            class=TA_CLASSES
            rows="4"
            placeholder=move || placeholder.get()
            bind:value=input
        ></textarea>
    }
}
