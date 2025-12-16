use crate::input_group::GroupItemClassContext;
use leptos::context::Provider;
use leptos::prelude::ChildrenFn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::slot;
use leptos::{IntoView, component, prelude::Children, view};

#[derive(Clone)]
pub struct InGroupContext {
    pub in_group: bool,
}

#[slot]
pub struct First {
    children: ChildrenFn,
}

#[slot]
pub struct Last {
    children: ChildrenFn,
}

/// Blocked on passing context to first and last child.
#[component]
pub fn ButtonGroup(
    first: First,
    #[prop(default=Box::new(move || { view!{}.into_any()}))] children: Children,
    last: Last,
) -> impl IntoView {
    view! {
        <div class="inline-flex rounded-lg shadow-sm -space-x-px" role="group">
            <Provider<InGroupContext, _> value=InGroupContext { in_group: true }>
                <Provider<GroupItemClassContext, _> value=GroupItemClassContext{ class: "rounded-l-lg".to_string() }>
                    {(first.children)().into_any()}
                </Provider<GroupItemClassContext, _>>
                {children()}
                <Provider<GroupItemClassContext, _> value=GroupItemClassContext{ class: "!border-r rounded-r-lg".to_string() }>
                    {(last.children)().into_any()}
                </Provider<GroupItemClassContext, _>>
            </Provider<InGroupContext, _>>
        </div>
    }
}
