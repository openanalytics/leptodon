use leptos::IntoView;
use leptos::prelude::AriaAttributes;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::MaybeProp;
use leptos::view;
use leptos::{component, prelude::Children};

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;

const HEADER_CLASS: &str = "font-bold relative text-gray-900 dark:text-gray-100";

#[component]
fn HeadingAnchor(
    /// Identifies this anchor.
    id: String
) -> impl IntoView {
    view! {
        // TODO: Get this offset via navbar context 
        <span id=id.clone() class="absolute -top-[100px]"></span>
        <a class="ml-2 text-oa-blue opacity-0 transition-opacity hover:opacity-100" href=format!("#{id}") aria-label="Link to this section: Heading mark">#</a>
    }
}

#[component]
fn MaybeHeadingAnchor(anchor: MaybeProp<String>) -> impl IntoView {
    if let Some(anchor) = anchor.get() {
        view !{ <HeadingAnchor id=anchor/> }.into_any()
    } else {
        ().into_any()
    }
}

#[component]
pub fn Heading1(
    /// Whether this Heading should be suffixed by a # serving as an anchor.
    #[prop(optional, into)]
    anchor: MaybeProp<String>,
    /// Extra heading classes
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// Heading contents
    children: Children,
) -> impl IntoView {
    view! {
        <h1 class=class_list!("text-5xl", HEADER_CLASS, class)>
            {children()}
            <MaybeHeadingAnchor anchor/>
        </h1>
    }
}

#[component]
pub fn Heading2(
    /// Whether this Heading should be suffixed by a # serving as an anchor.
    #[prop(optional, into)]
    anchor: MaybeProp<String>,
    /// Extra heading classes
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// Heading contents
    children: Children,
) -> impl IntoView {
    view! {
        <h2 class=class_list!("text-4xl", HEADER_CLASS, class)>
            {children()}
            <MaybeHeadingAnchor anchor/>
        </h2>
    }
}

#[component]
pub fn Heading3(
    /// Whether this Heading should be suffixed by a # serving as an anchor.
    #[prop(optional, into)]
    anchor: MaybeProp<String>,
    /// Extra heading classes
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// Heading contents
    children: Children,
) -> impl IntoView {
    view! {
        <h3 class=class_list!("text-3xl", HEADER_CLASS, class)>
            {children()}
            <MaybeHeadingAnchor anchor/>
        </h3>
    }
}

#[component]
pub fn Heading4(
    /// Whether this Heading should be suffixed by a # serving as an anchor.
    #[prop(optional, into)]
    anchor: MaybeProp<String>,
    /// Extra heading classes
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// Heading contents
    children: Children,
) -> impl IntoView {
    view! {
        <h4 class=class_list!("text-2xl", HEADER_CLASS, class)>
            {children()}
            <MaybeHeadingAnchor anchor/>
        </h4>
    }
}

#[component]
pub fn Heading5(
    /// Whether this Heading should be suffixed by a # serving as an anchor.
    #[prop(optional, into)]
    anchor: MaybeProp<String>,
    /// Extra heading classes
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// Heading contents
    children: Children,
) -> impl IntoView {
    view! {
        <h5 class=class_list!("text-xl", HEADER_CLASS, class)>
            {children()}
            <MaybeHeadingAnchor anchor/>
        </h5>
    }
}

#[component]
pub fn Heading6(
    /// Whether this Heading should be suffixed by a # serving as an anchor.
    #[prop(optional, into)]
    anchor: MaybeProp<String>,
    /// Extra heading classes
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// Heading contents
    children: Children,
) -> impl IntoView {
    view! {
        <h6 class=class_list!("text-lg", HEADER_CLASS, class)>
            {children()}
            <MaybeHeadingAnchor anchor/>
        </h6>
    }
}
