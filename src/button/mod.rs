use crate::button_group::InGroupContext;
use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;
use crate::icon::Icon;
use crate::icon::icon_data::IconRef;
use crate::input_group::GroupItemClassContext;
use crate::util::signals::ComponentRef;
use crate::{
    spinner::{Spinner, SpinnerSize},
    util::callback::BoxOneCallback,
};
use leptos::logging::debug_log;
use leptos::{IntoView, component, view};
use leptos::{
    either::{Either, EitherOf3},
    ev, html,
    prelude::*,
};

mod variations;

// Re-exports
pub use crate::button::variations::AddButton;
pub use crate::button::variations::ControlButton;
pub use crate::button::variations::DeleteButton;
pub use crate::button::variations::DialogButton;
pub use crate::button::variations::DialogButtonChildren;
pub use crate::button::variations::DownloadButton;
pub use crate::button::variations::DropdownButton;
pub use crate::button::variations::DropdownButtonChildren;
pub use crate::button::variations::EditButton;
pub use crate::button::variations::ModalButton;
pub use crate::button::variations::ModalButtonChildren;

const BUTTON_SHADOW_CLASSES: &str = "shadow-sm";
const BUTTON_SPACING_CLASSES: &str = " px-5 py-2.5 mr-2";
const SHARED_BUTTON_CLASSES: &str = "relative hover:z-20 focus:z-10 dark:focus:ring-gray-800 outline-offset-[-1px] outline-[5px] focus:outline font-medium inline-flex items-center text-center text-sm";

const BUTTON_GRAY_FOCUS_CLASSES: &str =
    "!active:outline-oa-gray-darker focus:outline-oa-gray-darker hover:focus:outline-oa-gray ";
// Light theme: dark text on light button; Dark theme: light text on dark button.
const BUTTON_DEFAULT_TEXT: &str = "text-gray-700 dark:text-gray-300";

const OA_PRIMARY_BUTTON_CLASSES: &str = const_str::join!(
    &[
        "focus:outline-oa-blue hover:bg-oa-blue-darker bg-oa-blue text-white",
        SHARED_BUTTON_CLASSES,
        BUTTON_SHADOW_CLASSES,
        BUTTON_SPACING_CLASSES
    ],
    " "
);

const OA_DANGER_BUTTON_CLASSES: &str = const_str::join!(
    &[
        "focus:outline-oa-red hover:bg-oa-red-darker bg-oa-red text-white",
        SHARED_BUTTON_CLASSES,
        BUTTON_SHADOW_CLASSES,
        BUTTON_SPACING_CLASSES
    ],
    " "
);

const OA_SECONDARY_BUTTON_CLASSES: &str = const_str::join!(
    &[
        "border-solid border border-gray-400",
        "!active:bg-oa-gray-darker bg-gray-200 hover:bg-oa-gray-darker !dark:active:bg-gray-600 dark:bg-gray-700 hover:dark:bg-gray-600",
        BUTTON_GRAY_FOCUS_CLASSES,
        BUTTON_DEFAULT_TEXT,
        SHARED_BUTTON_CLASSES,
        BUTTON_SHADOW_CLASSES,
        BUTTON_SPACING_CLASSES
    ],
    " "
);

pub const OA_TRANSPARENT_BUTTON_CLASSES: &str = const_str::join!(
    &[
        "hover:bg-oa-gray active:bg-oa-gray hover:dark:bg-gray-600 active:dark:bg-gray-600",
        SHARED_BUTTON_CLASSES,
        BUTTON_DEFAULT_TEXT,
        BUTTON_GRAY_FOCUS_CLASSES,
        BUTTON_SPACING_CLASSES
    ],
    " "
);

pub const OA_MINIMAL_BUTTON_CLASSES: &str = const_str::join!(
    &[
        SHARED_BUTTON_CLASSES,
        BUTTON_DEFAULT_TEXT,
        BUTTON_GRAY_FOCUS_CLASSES
    ],
    " "
);

#[component]
pub fn Button(
    /// Button ID
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Extra classes appened to the button's default style
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// A button can have its content and borders styled for greater emphasis or to be subtle.
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// A button supports different sizes.
    #[prop(optional, into)]
    size: MaybeProp<Signal<ButtonSize>>,
    /// The default behavior of the button.
    #[prop(optional, into)]
    button_type: MaybeProp<ButtonType>,
    /// The shape of the button.
    #[prop(default = ButtonShape::default(), into)]
    shape: ButtonShape,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<IconRef>,
    /// Whether the button shows the loading status.
    #[prop(optional, into)]
    loading: Signal<bool>,
    #[prop(optional, into)] on_click: Option<BoxOneCallback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] comp_ref: ComponentRef<ButtonRef>,
) -> impl IntoView
where
{
    let size_injection = ButtonSizeInjection::use_context().map(|s| s.0);
    let size = size
        .get()
        .unwrap_or_else(|| Signal::stored(size_injection.unwrap_or_default()));
    let in_group = use_context::<InGroupContext>().unwrap_or(InGroupContext { in_group: false });
    let aria_disabled = move || {
        if loading.get() { Some("true") } else { None }
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

    let group_context = use_context::<GroupItemClassContext>();
    let group_classes = group_context.map(|item| item.class);

    view! {
        <button
            id=move || id.get()
            class=class_list![
                class,
                group_classes.unwrap_or_default(),
                if in_group.in_group { "rounded-none border-r-0 !mr-0" } else { "" },
                match appearance.get() {
                    ButtonAppearance::Secondary => OA_SECONDARY_BUTTON_CLASSES,
                    ButtonAppearance::Primary => OA_PRIMARY_BUTTON_CLASSES,
                    ButtonAppearance::Danger => OA_DANGER_BUTTON_CLASSES,
                    ButtonAppearance::Subtle => todo!(),
                    ButtonAppearance::Transparent => OA_TRANSPARENT_BUTTON_CLASSES,
                    ButtonAppearance::Minimal => OA_MINIMAL_BUTTON_CLASSES,
                },
                match shape {
                    ButtonShape::Square => "rounded-none",
                    ButtonShape::Rounded => "rounded-lg",
                    ButtonShape::Circular => "rounded-full",
                }
            ]
            node_ref=button_ref
            type=move || button_type.get().map(|t| t.as_str())
            aria-disabled=aria_disabled
            on:click=on_click
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
                    EitherOf3::B(view!{
                        <Icon icon=icon class="w-5 h-5"/>
                    })
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
    /// Dangerous action
    Danger,
    /// Minimizes emphasis to blend into the background until hovered or focused.
    Subtle,
    /// Removes background and border styling.
    Transparent,
    /// Removes padding, margin, background and border styling.
    Minimal,
}

impl ButtonAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonAppearance::Secondary => "secondary",
            ButtonAppearance::Primary => "primary",
            ButtonAppearance::Subtle => "subtle",
            ButtonAppearance::Transparent => "transparent",
            ButtonAppearance::Danger => "danger",
            ButtonAppearance::Minimal => "minimal",
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonShape {
    #[default]
    /// Slightly rounded corners.
    Rounded,
    /// Fully rounded, will make a half-circle on the shortest side.
    /// Looks like a circle when all sides are equal.
    Circular,
    /// Pointy corners.
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
            button_el.click();
        } else {
            debug_log!("Button is missing! can't click");
        }
    }

    /// Focus the button element.
    pub fn focus(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            if let Err(err) = button_el.focus() {
                debug_log!("{err:?}");
            }
            debug_log!("Focused button");
        } else {
            debug_log!("Button is missing! can't focus");
        }
    }

    /// Blur the button element
    pub fn blur(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            if let Err(err) = button_el.blur() {
                debug_log!("{err:?}");
            }
            debug_log!("Blurred button");
        } else {
            debug_log!("Button is missing! can't blur");
        }
    }
}
