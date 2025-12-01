#![allow(non_snake_case)]
pub mod icon_data;

use crate::icon::icon_data::*;
use crate::icon_data;
use leptos::IntoView;
use leptos::component;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::ClassAttribute;
use leptos::prelude::CustomAttribute;
use leptos::prelude::Get;
use leptos::prelude::InnerHtmlAttribute;
use leptos::prelude::IntoAny;
use leptos::prelude::MaybeProp;
use leptos::prelude::OnAttribute;
use leptos::view;
use std::sync::LazyLock;

use leptos::ev;

use crate::class_list;
use crate::util::callback::ArcOneCallback;
/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: IconRef,
    /// The width of the icon (horizontal side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into, default = "1em".into())]
    width: MaybeProp<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into, default = "1em".into())]
    height: MaybeProp<String>,
    /// HTML class attribute.
    #[prop(into, default = "w-6 h-6".into())]
    class: MaybeProp<String>,
    /// Callback when clicking on the icon.
    #[prop(optional, into)]
    on_click: Option<ArcOneCallback<ev::MouseEvent>>,
) -> impl IntoView {
    let on_click = move |ev| {
        if let Some(click) = on_click.as_ref() {
            click(ev);
        }
    };

    view! {
        <svg
            class=class_list![icon.class.unwrap_or_default(), class]
            x=icon.x
            y=icon.y
            width=move || width.get()
            height=move || height.get()
            viewBox=icon.view_box
            stroke-linecap=icon.stroke_linecap
            stroke-linejoin=icon.stroke_linejoin
            stroke-width=icon.stroke_width
            stroke=icon.stroke
            fill=icon.fill.unwrap_or("none")
            inner_html=icon.data
            on:click=on_click
        ></svg>
    }.into_any()
}

/// Creates a [LazyLock] around a rendered version of the passed `html! {...}`.
macro_rules! lazy_path {
    ($name:ident, html! $vw:tt) => {
        static $name: LazyLock<String> = std::sync::LazyLock::new(|| rstml_to_string_macro::html!$vw);
    };
}

/// Creates a [LazyLock] around the passed IconData to obtain a reference with `'static` lifetime
macro_rules! return_lazified_icon {
    ($($stmt:stmt)*) => {
        static RET: LazyLock<IconData> = std::sync::LazyLock::new(|| {
            $($stmt)*
        });
        return &RET;
    }
}

pub fn HamburgerIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                clip-rule="evenodd"
                fill-rule="evenodd"
                d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn UserIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M10 8a3 3 0 100-6 3 3 0 000 6zM3.465 14.493a1.23 1.23 0 00.41 1.412A9.957 9.957 0 0010 18c2.31 0 4.438-.784 6.131-2.1.43-.333.604-.903.408-1.41a7.002 7.002 0 00-13.074.003z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn CalendarIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M12.75 12.75a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM7.5 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM8.25 17.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM9.75 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM10.5 17.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM12 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM12.75 17.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM14.25 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM15 17.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM16.5 15.75a.75.75 0 100-1.5.75.75 0 000 1.5zM15 12.75a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM16.5 13.5a.75.75 0 100-1.5.75.75 0 000 1.5z"></path>
            <path
                fill-rule="evenodd"
                d="M6.75 2.25A.75.75 0 017.5 3v1.5h9V3A.75.75 0 0118 3v1.5h.75a3 3 0 013 3v11.25a3 3 0 01-3 3H5.25a3 3 0 01-3-3V7.5a3 3 0 013-3H6V3a.75.75 0 01.75-.75zm13.5 9a1.5 1.5 0 00-1.5-1.5H5.25a1.5 1.5 0 00-1.5 1.5v7.5a1.5 1.5 0 001.5 1.5h13.5a1.5 1.5 0 001.5-1.5v-7.5z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn ProjectsIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13.5 8H4m0-2v13a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V9a1 1 0 0 0-1-1h-5.032a1 1 0 0 1-.768-.36l-1.9-2.28a1 1 0 0 0-.768-.36H5a1 1 0 0 0-1 1Z"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn ProjectsIconFilled() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M19.5 21a3 3 0 003-3v-4.5a3 3 0 00-3-3h-15a3 3 0 00-3 3V18a3 3 0 003 3h15zM1.5 10.146V6a3 3 0 013-3h5.379a2.25 2.25 0 011.59.659l2.122 2.121c.14.141.331.22.53.22H19.5a3 3 0 013 3v1.146A4.483 4.483 0 0019.5 9h-15a4.483 4.483 0 00-3 1.146z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn CustomersIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M4.5 2.25a.75.75 0 000 1.5v16.5h-.75a.75.75 0 000 1.5h16.5a.75.75 0 000-1.5h-.75V3.75a.75.75 0 000-1.5h-15zM9 6a.75.75 0 000 1.5h1.5a.75.75 0 000-1.5H9zm-.75 3.75A.75.75 0 019 9h1.5a.75.75 0 010 1.5H9a.75.75 0 01-.75-.75zM9 12a.75.75 0 000 1.5h1.5a.75.75 0 000-1.5H9zm3.75-5.25A.75.75 0 0113.5 6H15a.75.75 0 010 1.5h-1.5a.75.75 0 01-.75-.75zM13.5 9a.75.75 0 000 1.5H15A.75.75 0 0015 9h-1.5zm-.75 3.75a.75.75 0 01.75-.75H15a.75.75 0 010 1.5h-1.5a.75.75 0 01-.75-.75zM9 19.5v-2.25a.75.75 0 01.75-.75h4.5a.75.75 0 01.75.75v2.25a.75.75 0 01-.75.75h-4.5A.75.75 0 019 19.5z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn ContractsIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M18 5V4a1 1 0 0 0-1-1H8.914a1 1 0 0 0-.707.293L4.293 7.207A1 1 0 0 0 4 7.914V20a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5M9 3v4a1 1 0 0 1-1 1H4m11.383.772 2.745 2.746m1.215-3.906a2.089 2.089 0 0 1 0 2.953l-6.65 6.646L9 17.95l.739-3.692 6.646-6.646a2.087 2.087 0 0 1 2.958 0Z"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24));
}

pub fn ConsultantsIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M4.5 6.375a4.125 4.125 0 118.25 0 4.125 4.125 0 01-8.25 0zM14.25 8.625a3.375 3.375 0 116.75 0 3.375 3.375 0 01-6.75 0zM1.5 19.125a7.125 7.125 0 0114.25 0v.003l-.001.119a.75.75 0 01-.363.63 13.067 13.067 0 01-6.761 1.873c-2.472 0-4.786-.684-6.76-1.873a.75.75 0 01-.364-.63l-.001-.122zM17.25 19.128l-.001.144a2.25 2.25 0 01-.233.96 10.088 10.088 0 005.06-1.01.75.75 0 00.42-.643 4.875 4.875 0 00-6.957-4.611 8.586 8.586 0 011.71 5.157v.003z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn BillingIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M7 6a2 2 0 0 1 2-2h11a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2h-2v-4a3 3 0 0 0-3-3H7V6Z"
                clip-rule="evenodd"
            ></path>
            <path
                fill-rule="evenodd"
                d="M2 11a2 2 0 0 1 2-2h11a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-7Zm7.5 1a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5Z"
                clip-rule="evenodd"
            ></path>
            <path d="M10.5 14.5a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn TaskIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M17.133 12.632v-1.8a5.406 5.406 0 0 0-4.154-5.262.955.955 0 0 0 .021-.106V3.1a1 1 0 0 0-2 0v2.364a.955.955 0 0 0 .021.106 5.406 5.406 0 0 0-4.154 5.262v1.8C6.867 15.018 5 15.614 5 16.807 5 17.4 5 18 5.538 18h12.924C19 18 19 17.4 19 16.807c0-1.193-1.867-1.789-1.867-4.175ZM8.823 19a3.453 3.453 0 0 0 6.354 0H8.823Z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn ReportingIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M2 6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6Zm4.996 2a1 1 0 0 0 0 2h.01a1 1 0 1 0 0-2h-.01ZM11 8a1 1 0 1 0 0 2h6a1 1 0 1 0 0-2h-6Zm-4.004 3a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2h-.01ZM11 11a1 1 0 1 0 0 2h6a1 1 0 1 0 0-2h-6Zm-4.004 3a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2h-.01ZM11 14a1 1 0 1 0 0 2h6a1 1 0 1 0 0-2h-6Z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn ApproveIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M15.03 9.684h3.965c.322 0 .64.08.925.232.286.153.532.374.717.645a2.109 2.109 0 0 1 .242 1.883l-2.36 7.201c-.288.814-.48 1.355-1.884 1.355-2.072 0-4.276-.677-6.157-1.256-.472-.145-.924-.284-1.348-.404h-.115V9.478a25.485 25.485 0 0 0 4.238-5.514 1.8 1.8 0 0 1 .901-.83 1.74 1.74 0 0 1 1.21-.048c.396.13.736.397.96.757.225.36.32.788.269 1.211l-1.562 4.63ZM4.177 10H7v8a2 2 0 1 1-4 0v-6.823C3 10.527 3.527 10 4.176 10Z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn RejectIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M8.97 14.316H5.004c-.322 0-.64-.08-.925-.232a2.022 2.022 0 0 1-.717-.645 2.108 2.108 0 0 1-.242-1.883l2.36-7.201C5.769 3.54 5.96 3 7.365 3c2.072 0 4.276.678 6.156 1.256.473.145.925.284 1.35.404h.114v9.862a25.485 25.485 0 0 0-4.238 5.514c-.197.376-.516.67-.901.83a1.74 1.74 0 0 1-1.21.048 1.79 1.79 0 0 1-.96-.757 1.867 1.867 0 0 1-.269-1.211l1.562-4.63ZM19.822 14H17V6a2 2 0 1 1 4 0v6.823c0 .65-.527 1.177-1.177 1.177Z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn CloseIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 14).filled());
}

pub fn SearchIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20));
}

pub fn PreviousIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn NextIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn DownIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m1 1 4 4 4-4"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 10, 6));
}

pub fn RefreshIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M17.651 7.65a7.131 7.131 0 0 0-12.68 3.15M18.001 4v4h-4m-7.652 8.35a7.13 7.13 0 0 0 12.68-3.15M6 20v-4h4"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24));
}

pub fn DecrementIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M1 1h16"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 18, 2));
}

pub fn IncrementIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 1v16M1 9h16"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 18));
}

pub fn NoteFilledIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M3.559 4.544c.355-.35.834-.544 1.33-.544H19.11c.496 0 .975.194 1.33.544.356.35.559.829.559 1.331v9.25c0 .502-.203.981-.559 1.331-.355.35-.834.544-1.33.544H15.5l-2.7 3.6a1 1 0 0 1-1.6 0L8.5 17H4.889c-.496 0-.975-.194-1.33-.544A1.868 1.868 0 0 1 3 15.125v-9.25c0-.502.203-.981.559-1.331ZM7.556 7.5a1 1 0 1 0 0 2h8a1 1 0 0 0 0-2h-8Zm0 3.5a1 1 0 1 0 0 2H12a1 1 0 1 0 0-2H7.556Z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn NoteIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M7.556 8.5h8m-8 3.5H12m7.111-7H4.89a.896.896 0 0 0-.629.256.868.868 0 0 0-.26.619v9.25c0 .232.094.455.26.619A.896.896 0 0 0 4.89 16H9l3 4 3-4h4.111a.896.896 0 0 0 .629-.256.868.868 0 0 0 .26-.619v-9.25a.868.868 0 0 0-.26-.619.896.896 0 0 0-.63-.256Z"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24));
}

pub fn ProjectAssignmentsIconFilled() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M12 6a3.5 3.5 0 1 0 0 7 3.5 3.5 0 0 0 0-7Zm-1.5 8a4 4 0 0 0-4 4 2 2 0 0 0 2 2h7a2 2 0 0 0 2-2 4 4 0 0 0-4-4h-3Zm6.82-3.096a5.51 5.51 0 0 0-2.797-6.293 3.5 3.5 0 1 1 2.796 6.292ZM19.5 18h.5a2 2 0 0 0 2-2 4 4 0 0 0-4-4h-1.1a5.503 5.503 0 0 1-.471.762A5.998 5.998 0 0 1 19.5 18ZM4 7.5a3.5 3.5 0 0 1 5.477-2.889 5.5 5.5 0 0 0-2.796 6.293A3.501 3.501 0 0 1 4 7.5ZM7.1 12H6a4 4 0 0 0-4 4 2 2 0 0 0 2 2h.5a5.998 5.998 0 0 1 3.071-5.238A5.505 5.505 0 0 1 7.1 12Z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn ProjectAssignmentsIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-width="2"
                d="M4.5 17H4a1 1 0 0 1-1-1 3 3 0 0 1 3-3h1m0-3.05A2.5 2.5 0 1 1 9 5.5M19.5 17h.5a1 1 0 0 0 1-1 3 3 0 0 0-3-3h-1m0-3.05a2.5 2.5 0 1 0-2-4.45m.5 13.5h-7a1 1 0 0 1-1-1 3 3 0 0 1 3-3h3a3 3 0 0 1 3 3 1 1 0 0 1-1 1Zm-1-9.5a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0Z"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24));
}

pub fn LockOpenIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M15 7a2 2 0 1 1 4 0v4a1 1 0 1 0 2 0V7a4 4 0 0 0-8 0v3H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2V7Zm-5 6a1 1 0 0 1 1 1v3a1 1 0 1 1-2 0v-3a1 1 0 0 1 1-1Z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn LockIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M8 10V7a4 4 0 1 1 8 0v3h1a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h1Zm2-3a2 2 0 1 1 4 0v3h-4V7Zm2 6a1 1 0 0 1 1 1v3a1 1 0 1 1-2 0v-3a1 1 0 0 1 1-1Z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn ApprovedIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M8.5 11.5 11 14l4-4m6 2a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
            ></path>
        }
    );
    return_lazified_icon! {
        icon_data!(&HTML, 24)
            .set_class(Some("text-oa-blue dark:text-white"))
    };
}

pub fn RejectedIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m15 9-6 6m0-6 6 6m6-3a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
            ></path>
        }
    );
    return_lazified_icon! {
        icon_data!(&HTML, 24)
            .set_class(Some("text-red-600 dark:text-red-500"))
    };
}

pub fn PendingApprovalIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9.529 9.988a2.502 2.502 0 1 1 5 .191A2.441 2.441 0 0 1 12 12.582V14m-.01 3.008H12M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
            ></path>
        }
    );
    return_lazified_icon! {
        icon_data!(&HTML, 24)
            .set_class(Some("text-yellow-300 dark:text-yellow-300"))
    };
}

pub fn SavedIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M8.5 11.5 11 14l4-4m6 2a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
            ></path>
        }
    );
    return_lazified_icon! {
        icon_data!(&HTML, 24)
            .set_class(Some("text-oa-blue dark:text-oa-blue"))
    };
}

pub fn EmptyIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke-linecap="round"
                d="M7.757 12h8.486M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                stroke-linejoin="round"
                stroke="currentColor"
                stroke-width="2"
            ></path>
        }
    );
    return_lazified_icon! {
        icon_data!(&HTML, 24)
            .set_class(Some("text-gray-400 dark:text-gray-400"))
    };
}

pub fn OpenIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke-linejoin="round"
                stroke="currentColor"
                stroke-width="2"
                d="M12 13V8m0 8h.01M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                stroke-linecap="round"
            ></path>
        }
    );
    return_lazified_icon! {
        icon_data!(&HTML, 24)
            .set_class(Some("text-yellow-300 dark:text-yellow-300"))
    };
}

pub fn InfoIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn AutoFillIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 12H5m14 0-4 4m4-4-4-4"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn SaveIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn DateIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M20 4a2 2 0 0 0-2-2h-2V1a1 1 0 0 0-2 0v1h-3V1a1 1 0 0 0-2 0v1H6V1a1 1 0 0 0-2 0v1H2a2 2 0 0 0-2 2v2h20V4ZM0 18a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8H0v10Zm5-8h10a1 1 0 0 1 0 2H5a1 1 0 0 1 0-2Z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn TableSearchIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn UnsortedIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn SortedAscendingIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M14.77 12.79a.75.75 0 01-1.06-.02L10 8.832 6.29 12.77a.75.75 0 11-1.08-1.04l4.25-4.5a.75.75 0 011.08 0l4.25 4.5a.75.75 0 01-.02 1.06z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn SortedDescendingIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn HideIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-width="1.5"
                d="M3.933 13.909A4.357 4.357 0 0 1 3 12c0-1 4-6 9-6m7.6 3.8A5.068 5.068 0 0 1 21 12c0 1-3 6-9 6-.314 0-.62-.014-.918-.04M5 19 19 5m-4 7a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24));
}

pub fn ShowIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-width="1.5"
                d="M21 12c0 1.2-4.03 6-9 6s-9-4.8-9-6c0-1.2 4.03-6 9-6s9 4.8 9 6Z"
            ></path>
            <path
                stroke="currentColor"
                stroke-width="1.5"
                d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24));
}

pub fn MoveIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-width="2"
                d="M12 6h.01M12 12h.01M12 18h.01"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24));
}

pub fn EditIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M2.695 14.763l-1.262 3.154a.5.5 0 00.65.65l3.155-1.262a4 4 0 001.343-.885L17.5 5.5a2.121 2.121 0 00-3-3L3.58 13.42a4 4 0 00-.885 1.343z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn DeleteIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M8.75 1A2.75 2.75 0 006 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 10.23 1.482l.149-.022.841 10.518A2.75 2.75 0 007.596 19h4.807a2.75 2.75 0 002.742-2.53l.841-10.52.149.023a.75.75 0 00.23-1.482A41.03 41.03 0 0014 4.193V3.75A2.75 2.75 0 0011.25 1h-2.5zM10 4c.84 0 1.673.025 2.5.075V3.75c0-.69-.56-1.25-1.25-1.25h-2.5c-.69 0-1.25.56-1.25 1.25v.325C8.327 4.025 9.16 4 10 4zM8.58 7.72a.75.75 0 00-1.5.06l.3 7.5a.75.75 0 101.5-.06l-.3-7.5zm4.34.06a.75.75 0 10-1.5-.06l-.3 7.5a.75.75 0 101.5.06l.3-7.5z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn WarningIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 11V6m0 8h.01M19 10a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
            ></path>
        }
    );
    return_lazified_icon! {
        icon_data!(&HTML, 20)
            .set_class(Some("text-gray-400 dark:text-gray-200"))
    };
}

pub fn CancelIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn AddIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z"></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn FirstIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M15.79 14.77a.75.75 0 01-1.06.02l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 111.04 1.08L11.832 10l3.938 3.71a.75.75 0 01.02 1.06zm-6 0a.75.75 0 01-1.06.02l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 111.04 1.08L5.832 10l3.938 3.71a.75.75 0 01.02 1.06z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn LastIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M10.21 14.77a.75.75 0 01.02-1.06L14.168 10 10.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                clip-rule="evenodd"
            ></path>
            <path
                fill-rule="evenodd"
                d="M4.21 14.77a.75.75 0 01.02-1.06L8.168 10 4.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 20).filled());
}

pub fn DownloadIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                fill-rule="evenodd"
                d="M13 11.15V4a1 1 0 1 0-2 0v7.15L8.78 8.374a1 1 0 1 0-1.56 1.25l4 5a1 1 0 0 0 1.56 0l4-5a1 1 0 1 0-1.56-1.25L13 11.15Z"
                clip-rule="evenodd"
            ></path>
            <path
                fill-rule="evenodd"
                d="M9.657 15.874 7.358 13H5a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-2.358l-2.3 2.874a3 3 0 0 1-4.685 0ZM17 16a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17Z"
                clip-rule="evenodd"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24).filled());
}

pub fn ClearIcon() -> IconRef {
    lazy_path!(
        HTML,
        html! {
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18 17.94 6M18 18 6.06 6"
            ></path>
        }
    );
    return_lazified_icon!(icon_data!(&HTML, 24));
}
