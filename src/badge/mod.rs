use crate::avatar::Avatar;
use crate::class_list;
use crate::icon::Icon;
use crate::icon::icon_data::IconRef;
use crate::util::callback::ArcOneCallback;
use attr_docgen::generate_docs;
use leptos::either::Either;
use leptos::prelude::{AnyView, Children, ClassAttribute, ElementChild, IntoAny, MaybeProp};
use leptos::prelude::{AriaAttributes, CustomAttribute};
use leptos::prelude::{Get, OnAttribute, RwSignal, Set};
use leptos::{IntoView, component, view};
use web_sys::MouseEvent;

const BADGE_BASE_CLASSES: &str = "flex font-medium rounded-lg h-fit w-fit";

#[generate_docs]
/// A badge
#[component]
pub fn Badge(
    #[prop(optional)] size: BadgeSize,
    /// Extra styling
    #[prop(optional, into)]
    class: MaybeProp<String>,
    #[prop(optional)] theme: BadgeTheme,
    /// Shown inside the badge, before the children
    #[prop(optional, into)]
    prefix: MaybeProp<BadgePrefix>,
    /// Shown inside the badge, after the children
    #[prop(optional, into)]
    postfix: MaybeProp<BadgePostfix>,
    /// Whether the badge should have a border
    #[prop(optional)]
    border: bool,
    /// Whether the badge should have a cross button to dismiss it.
    #[prop(optional)]
    dismissable: bool,
    /// Ran on dismissal request
    #[prop(optional)]
    on_dismiss: Option<ArcOneCallback<MouseEvent>>,
    /// Usually text content.
    children: Children,
) -> impl IntoView {
    let prefix_classes = move |prefix: MaybeProp<BadgePrefix>| {
        prefix.get().map(|prefix| prefix.class()).unwrap_or("")
    };
    let dismissed = RwSignal::new(false);
    view! {
        <span
            class=class_list!(
                BADGE_BASE_CLASSES, class, theme.base_class(),
                (theme.border_class(), border),
                ("gap-1", dismissable),
                ("hidden", move || dismissed.get()),
                prefix_classes(prefix), prefix_classes(postfix), size.class()
            )
        >
            {move || if let Some(prefix) = prefix.get() {
                prefix.view()
            } else {
                ().into_any()
            }}
            <span>{children()}</span>
            {move || if let Some(postfix) = postfix.get() {
                postfix.view()
            } else {
                ().into_any()
            }}
            {if dismissable {
                Either::Left(view!{
                    <button type="button" class="inline-flex items-center p-0.5 text-sm bg-transparent rounded-xs hover:bg-neutral-tertiary" aria_label="Remove" on:click=move |e| {
                        if let Some(on_dismiss) = on_dismiss.clone() {
                            on_dismiss(e);
                        }
                        dismissed.set(true);
                    }>
                        <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18 17.94 6M18 18 6.06 6"/></svg>
                        <span class="sr-only">Remove badge</span>
                    </button>
                })
            } else {
                Either::Right(())
            }}
        </span>
    }
}

#[derive(Default)]
pub enum BadgeSize {
    #[default]
    Normal,
    Large,
}

impl BadgeSize {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeSize::Normal => "text-xs px-1 py-0.5",
            BadgeSize::Large => "text-sm px-2 py-1",
        }
    }
}

#[derive(Default)]
pub enum BadgeTheme {
    #[default]
    Brand,
    Secondary,
    Transparent,
    Danger,
    Success,
    Warning,
}

impl BadgeTheme {
    // Color theme
    pub fn base_class(&self) -> &'static str {
        match self {
            BadgeTheme::Brand => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
            BadgeTheme::Secondary => {
                "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
            }
            BadgeTheme::Transparent => "dark:text-white",
            BadgeTheme::Danger => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
            BadgeTheme::Success => {
                "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300"
            }
            BadgeTheme::Warning => {
                "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300"
            }
        }
    }

    // Border and their colors
    pub fn border_class(&self) -> &'static str {
        match self {
            BadgeTheme::Brand => "border border-blue-400",
            BadgeTheme::Secondary => "border border-gray-400",
            BadgeTheme::Transparent => "border border-gray-400",
            BadgeTheme::Danger => "border border-red-400",
            BadgeTheme::Success => "border border-green-400",
            BadgeTheme::Warning => "border border-yellow-400",
        }
    }
}

pub type BadgePostfix = BadgePrefix;
#[derive(Clone)]
pub enum BadgePrefix {
    Icon(IconRef),
    Dot,
    SvgLoader,
    Avatar {
        /// Url or data of the avatar image
        src: String,
    },
}
impl BadgePrefix {
    pub fn class(self) -> &'static str {
        "inline-flex items-center"
    }

    pub fn view(&self) -> AnyView {
        let size = "h-[1lh] w-[1lh] me-1";
        match self {
            BadgePrefix::Icon(icon_data) => view! {
                <Icon class=class_list!(size) icon=*icon_data/>
            }.into_any(),
            BadgePrefix::Dot => view! {
                <span class=class_list!("w-1.5 h-1.5 me-1 bg-oa-blue rounded-full")></span>
            }.into_any(),
            BadgePrefix::SvgLoader => view! {
                <svg aria-hidden="true" role="status" class=class_list!(size, "animate-spin text-black fill-oa-gray dark:fill-black dark:text-oa-gray") viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                </svg>
            }.into_any(),
            BadgePrefix::Avatar { src } => view! {
                <Avatar src=src.to_string() class=class_list!(size) />
            }.into_any()
        }
    }
}
