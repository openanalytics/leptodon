use leptos::prelude::AddAnyAttr;
use leptos_router::components::ToHref;
use leptos::{IntoView, component, oco::Oco, prelude::Children, view};
use leptos_router::components::A;

const LINK_CLASSES: &str = "text-oa-blue";

/// See [[A](leptos_router::components::A)]
#[component]
pub fn Link<H>(
    /// Used to calculate the link's `href` attribute. Will be resolved relative
    /// to the current route.
    href: H,
    /// Where to display the linked URL, as the name for a browsing context (a tab, window, or `<iframe>`).
    #[prop(default = Oco::Borrowed(""), into)]
    target: Oco<'static, str>,
    /// If `true`, the link is marked aria-active when the location matches exactly;
    /// if false, link is marked aria-active if the current route starts with it.
    #[prop(optional)]
    exact: bool,
    /// If `true`, and when `href` has a trailing slash, `aria-current` be only be set if `current_url` also has
    /// a trailing slash.
    #[prop(optional)]
    strict_trailing_slash: bool,
    /// If `true`, the router will scroll to the top of the window at the end of navigation. Defaults to `true`.
    #[prop(default = true)]
    scroll: bool,
    /// The nodes or elements to be shown inside the link.
    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
        <A href target exact strict_trailing_slash scroll {..} class=LINK_CLASSES>
            {children()}
        </A>
    }
}