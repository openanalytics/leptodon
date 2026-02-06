use leptos::IntoView;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;
use leptos::{component, prelude::Children};

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;

/// A flex column which centers its items.
#[component]
pub fn CenteringColumn(
    #[prop(optional, into)] class: MaybeReactiveClass,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!("flex flex-col justify-center items-center", class)>
            {children()}
        </div>
    }
}

/// Column with a fixed width
#[component]
pub fn FixedColumn(
    #[prop(optional, into)] class: MaybeReactiveClass,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!("flex flex-col w-full lg:w-[770px] xl:w-[1024px]", class)>
            {children()}
        </div>
    }
}

/// A fixed column but centered.
#[component]
pub fn FixedCenterColumn(
    #[prop(optional, into)] class: MaybeReactiveClass,
    children: Children,
) -> impl IntoView {
    view! {
        <CenteringColumn class>
            <FixedColumn>
                {children()}
            </FixedColumn>
        </CenteringColumn>
    }
}
