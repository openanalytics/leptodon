
use crate::class_list;
use leptos::leptos_dom::logging::console_log;
use leptos::{IntoView, component, view};
use leptos::
    prelude::*
;

use std::sync::atomic::{AtomicU64, Ordering};

const ATOMIC_ACCORDION_ID: AtomicU64 = AtomicU64::new(1);
const ACCORDION_HEADING_CLASSES: &str = "flex items-center justify-between w-full p-5 font-medium rtl:text-right text-body rounded-t-base border border-t-0 border-x-0 border-b-default hover:text-heading hover:bg-neutral-secondary-medium gap-3";
const ACCORDION_BODY_CLASSES: &str =
    "border border-s-0 border-e-0 border-t-0 border-b-default";

/// Single collapseable section for the [Accordion]
#[component]
pub fn AccordionEntry(
    /// Header title that opens/closes the accordion entry upon a click.
    #[prop(into)]
    title: String,
    /// Normally a set of paragraphs with the text-body class.
    children: Children,
) -> impl IntoView {
    let id = ATOMIC_ACCORDION_ID.fetch_add(1, Ordering::SeqCst);
    let head_id = format!("accordion-collapse-heading-{id}");
    let body_id = format!("accordion-collapse-body-{id}");
    let (is_hidden, set_hidden) = signal(true);
    view! {
        <h2 id=head_id.clone()>
          <button type="button" class=ACCORDION_HEADING_CLASSES aria-expanded="true" aria-controls=body_id.clone()
            on:click=move |_| {
                console_log(format!("Is_hidden: {}", is_hidden.get()).as_str());
                set_hidden.set(!is_hidden.get()); 
            }>
            <span>{ title }</span>
            <svg class="w-5 h-5 rotate-180 shrink-0" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m5 15 7-7 7 7"/></svg>
          </button>
        </h2>
        <div id=body_id class=class_list![ACCORDION_BODY_CLASSES, ("hidden", move || is_hidden.get())] aria-labelledby=head_id>
          <div class="p-4 md:p-5">
            { children() }
          </div>
        </div>
    }
}

/// A stack of expandable/collapseable sections.
#[component]
pub fn Accordion<T>(
    /// A set accordion entries, see [AccordionEntry]
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    view! {
        <div id="accordion-collapse" data-accordion="collapse" class="rounded-base border border-default overflow-hidden shadow-xs">
            { children.into_inner()() }
        </div>
    }
}
