use attr_docgen::generate_docs;
use leptos::prelude::BindAttribute;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::Signal;
use leptos::prelude::use_context;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, RwSignal},
    view,
};
use leptos_use::math::use_or;

use crate::class_list;
use crate::form_input::FormInputContext;
use crate::form_input::Label;

const TA_CLASSES: &str = "block p-2.5 w-full resize text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
const LABEL_CLASSES: &str = "block mb-2 text-sm font-medium text-gray-900 dark:text-white";

#[generate_docs]
#[component]
pub fn TextArea(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    value: RwSignal<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    // Form context
    let form_context = use_context::<FormInputContext<String>>();
    let form_required = Signal::from(
        form_context
            .clone()
            .map(|ctx| ctx.required)
            .unwrap_or_default(),
    );
    let required = use_or(required, form_required);
    view! {
        <Label label required=required.get()>
            <textarea
                id=id.get()
                name=name.get()
                class=class_list![TA_CLASSES, class]
                rows="4"
                placeholder=move || placeholder.get()
                required=required
                bind:value=value
            ></textarea>
        </Label>
    }
}
