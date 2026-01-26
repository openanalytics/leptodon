use std::marker::PhantomData;

use leptos::context::Provider;
use leptos::prelude::{ClassAttribute, ElementChild, Get, RwSignal, Show};
use leptos::{IntoView, component, prelude::Children, view};

#[derive(Clone, Copy)]
pub struct FormInputContext<E: Clone + Send + Sync + std::fmt::Display + 'static> {
    pub required: bool,
    pub feedback: RwSignal<Option<E>>,
}

/// Use this to add labels, required-indicator and automatic feedback to form inputs (crate::input, crate::select, crate::radio).
#[component]
pub fn FormInput<E>(
    #[prop(into)] label: String,
    required: bool,
    children: Children,
    #[prop(default = PhantomData)] _phantom: PhantomData<E>,
) -> impl IntoView
where
    E: Clone + Send + Sync + std::fmt::Display + 'static,
{
    let feedback: RwSignal<Option<E>> = RwSignal::new(None);
    view! {
        <div class="mb-2">
            <Label label required>
                <Provider<_, _> value=FormInputContext { required, feedback }>
                    {children()}
                </Provider<_, _>>
            </Label>

            // Feedback
            <Show
                when=move || feedback.get().is_some()
                fallback=|| ()
            >
                <span class="text-red-500">{
                    move || feedback.get().unwrap().to_string()
                }</span>
            </Show>
        </div>
    }
}

#[component]
pub fn Label(label: String, required: bool, children: Children) -> impl IntoView {
    view! {
        <label class="block text-sm font-medium text-heading">
            <div>
                <RequiredStar required/>
                {label}
            </div>
            {children()}
        </label>
    }
}

#[component]
pub fn RequiredStar(required: bool) -> impl IntoView {
    view! {
        <Show
            when=move || required
            fallback=|| ()
        >
            <span class="text-red-500">*</span>
        </Show>
    }
}
