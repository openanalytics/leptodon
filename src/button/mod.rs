use crate::class_list;
use crate::util::signals::ComponentRef;
use crate::{
    icon::Icon,
    spinner::{Spinner, SpinnerSize},
    util::callback::BoxOneCallback,
};
use leptos::{IntoView, component, view};
use leptos::{
    either::{Either, EitherOf3},
    ev, html,
    prelude::*,
};
// const PRIMARY_BUTTON_CLASSES: &'static str = "text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none";
const OA_PRIMARY_BUTTON_CLASSES: &'static str = "dark:focus:ring-gray-800 outline-offset-[-1px] outline-5 focus:outline focus:outline-oa-blue font-medium inline-flex items-center text-center text-sm rounded-lg px-5 py-2.5 mr-2 hover:bg-oa-blue-darker bg-oa-blue text-white";
const OA_SECONDARY_BUTTON_CLASSES: &'static str = "dark:focus:ring-gray-800 focus:outline-none focus:ring-4 focus:ring-oa-gray font-medium inline-flex items-center text-center text-sm rounded-lg px-5 py-2.5 mr-2 bg-gray-200 hover:bg-oa-gray-darker text-gray-700 dark:bg-gray-700 dark:text-gray-400";

/// A button triggers an action or event when activated.
#[component]
pub fn Button(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A button can have its content and borders styled for greater emphasis or to be subtle.
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// A button can be rounded, circular, or square.
    #[prop(optional, into)]
    shape: Signal<ButtonShape>,
    /// A button supports different sizes.
    #[prop(optional, into)]
    size: Option<Signal<ButtonSize>>,
    /// The default behavior of the button.
    #[prop(optional, into)]
    button_type: MaybeProp<ButtonType>,
    /// Whether the button is displayed as block.
    #[prop(optional, into)]
    block: Signal<bool>,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    /// Whether the button shows the loading status.
    #[prop(optional, into)]
    loading: Signal<bool>,
    #[prop(optional, into)] on_click: Option<BoxOneCallback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] comp_ref: ComponentRef<ButtonRef>,
) -> impl IntoView {
    let none_children = children.is_none();
    let size_injection = ButtonSizeInjection::use_context().map(|s| s.0);
    let size = size.unwrap_or_else(|| Signal::stored(size_injection.unwrap_or_default()));
    let only_icon = Memo::new(move |_| icon.with(|i| i.is_some()) && none_children);
    let aria_disabled = move || {
        if loading.get() {
            return Some("true");
        } else {
            return None;
        }
    };

    let button_ref = NodeRef::<html::Button>::new();
    comp_ref.load(ButtonRef { button_ref });

    let on_click = move |e| {
        // if btn_disabled.get_untracked() {
        //     return;
        // }
        if loading.get_untracked() {
            return;
        }

        let Some(on_click) = on_click.as_ref() else {
            return;
        };
        on_click(e);
    };
    view! {
        <button
            class=class_list![
                "thaw-button",

                ("thaw-button--block", move || block.get()),
                ("thaw-button--only-icon", only_icon),
                ("thaw-button--icon", move || icon.with(|i| i.is_some())),
                ("thaw-button--loading", move || loading.get()),
                move || format!("thaw-button--{}", size.get().as_str()),
                move || format!("thaw-button--{}", appearance.get().as_str()),
                move || format!("thaw-button--{}", shape.get().as_str()),
                match appearance.get() {
                    ButtonAppearance::Secondary => OA_SECONDARY_BUTTON_CLASSES,
                    ButtonAppearance::Primary => OA_PRIMARY_BUTTON_CLASSES,
                    ButtonAppearance::Subtle => "",
                    ButtonAppearance::Transparent => todo!(),
                },
                class
            ]
            type=move || button_type.get().map(|t| t.as_str())
            aria-disabled=aria_disabled
            on:click=move |e| on_click(e)
        >
            {move || {
                if loading.get() {
                    EitherOf3::A(
                        view! {
                            <span class="thaw-button__icon">
                                <Spinner size=Signal::derive(move || size.get().into()) />
                            </span>
                        },
                    )
                } else if let Some(icon) = icon.get() {
                    EitherOf3::B(view! { <Icon icon=icon class="thaw-button__icon" /> })
                } else {
                    EitherOf3::C(())
                }
            }}
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </button>
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonAppearance {
    /// Gives emphasis to the button in such a way that it indicates a secondary action.
    #[default]
    Secondary,
    /// Emphasizes the button as a primary action.
    Primary,
    /// Minimizes emphasis to blend into the background until hovered or focused.
    Subtle,
    /// Removes background and border styling.
    Transparent,
}

impl ButtonAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonAppearance::Secondary => "secondary",
            ButtonAppearance::Primary => "primary",
            ButtonAppearance::Subtle => "subtle",
            ButtonAppearance::Transparent => "transparent",
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonShape {
    #[default]
    Rounded,
    Circular,
    Square,
}

impl ButtonShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonShape::Rounded => "rounded",
            ButtonShape::Circular => "circular",
            ButtonShape::Square => "square",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Small => "small",
            ButtonSize::Medium => "medium",
            ButtonSize::Large => "large",
        }
    }
}

impl From<ButtonSize> for SpinnerSize {
    fn from(value: ButtonSize) -> Self {
        match value {
            ButtonSize::Small => Self::Tiny,
            ButtonSize::Medium => Self::Tiny,
            ButtonSize::Large => Self::ExtraSmall,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ButtonSizeInjection(pub ButtonSize);

impl ButtonSizeInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}

/// The default behavior of the button.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#type)
#[derive(Debug, Clone)]
pub enum ButtonType {
    /// The button submits the form data to the server.
    /// This is the default if the attribute is not specified for buttons associated with a <form>,
    /// or if the attribute is an empty or invalid value.
    Submit,
    /// The button resets all the controls to their initial values,
    /// like <input type="reset">. (This behavior tends to annoy users.)
    Reset,
    /// The button has no default behavior, and does nothing when pressed by default.
    /// It can have client-side scripts listen to the element's events,
    /// which are triggered when the events occur.
    Button,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Button => "button",
        }
    }
}

#[derive(Clone)]
pub struct ButtonRef {
    pub(super) button_ref: NodeRef<html::Button>,
}

impl ButtonRef {
    /// Click the button element.
    pub fn click(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            _ = button_el.click();
        }
    }

    /// Focus the button element.
    pub fn focus(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            _ = button_el.focus();
        }
    }

    /// Blur the button element
    pub fn blur(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            _ = button_el.blur()
        }
    }
}
