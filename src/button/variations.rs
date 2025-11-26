use crate::button::ButtonProps;
use crate::button::{Button, ButtonAppearance, ButtonRef, ButtonSize, ButtonType};
use crate::dropdown::Dropdown;
use crate::icon::icon_data::IconRef;
use crate::util::callback::BoxOneCallback;
use crate::util::signals::ComponentRef;
use leptos::prelude::{Children, Get, MaybeProp, Signal, Write, provide_context, signal};
use leptos::slot;
use leptos::{IntoView, component, view};

#[slot]
pub struct DropdownButtonChildren {
    children: Children,
}

/// A button triggers an action or event when activated.
#[component]
pub fn DropdownButton(
    /// Extra classes appened to the button's default style
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// A button can have its content and borders styled for greater emphasis or to be subtle.
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// A button supports different sizes.
    #[prop(optional, into)]
    size: MaybeProp<Signal<ButtonSize>>,
    /// The default behavior of the button.
    #[prop(optional, into)]
    button_type: MaybeProp<ButtonType>,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<IconRef>,
    /// Whether the button shows the loading status.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Autoclose the dropdown upon clicking a dropdown item.
    #[prop(optional, default = true, into)]
    should_autoclose: bool,
    /// Most likely a label
    button_children: DropdownButtonChildren,
    /// Dropdown items
    children: Children,
    /// comp_ref will be filled with a reference to the DOM element.
    #[prop(optional)]
    comp_ref: ComponentRef<ButtonRef>,
) -> impl IntoView
where
{
    let (is_visible, set_visible) = signal(false);

    let button = Button(ButtonProps {
        class,
        appearance,
        size,
        button_type,
        icon,
        loading,
        on_click: Some(BoxOneCallback::new(move |_e| {
            *set_visible.write() = !is_visible.get();
        })),
        children: Some(button_children.children),
        comp_ref,
    });
    provide_context::<crate::dropdown::SetVisibleCallback>(set_visible);
    provide_context::<crate::dropdown::AutoClose>(should_autoclose);
    view! {
        {button}
        <Dropdown is_visible=is_visible>
            {children()}
        </Dropdown>
    }
}
