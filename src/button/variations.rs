use crate::button::ButtonProps;
use crate::button::ButtonShape;
use crate::button::{Button, ButtonAppearance, ButtonRef, ButtonSize, ButtonType};
use crate::class_list;
use crate::dialog::Dialog;
use crate::dropdown::AlignmentAnchor;
use crate::dropdown::Dropdown;
use crate::icon;
use crate::icon::icon_data::IconData;
use crate::icon::icon_data::IconRef;
use crate::modal::{Modal, ModalFooterChildren};
use crate::util::callback::BoxCallback;
use crate::util::callback::BoxOneCallback;
use crate::util::signals::ComponentRef;
use leptos::prelude::ClassAttribute;
use leptos::prelude::IntoAny;
use leptos::prelude::{Children, Get, MaybeProp, Signal, provide_context, signal};
use leptos::prelude::{ElementChild, RwSignal, Update};
use leptos::{IntoView, component, view};
use leptos::{ev, slot};

/// An icon only button meant for controlling another view (e.g. < > << >>)
#[component]
pub fn ControlButton(
    /// Button ID
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Extra classes appened to the button's default style
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// The icon of the button.
    #[prop(into)]
    icon: IconRef,
    #[prop(into)] on_click: BoxOneCallback<ev::MouseEvent>,
    #[prop(optional)] comp_ref: ComponentRef<ButtonRef>,
) -> impl IntoView {
    view! {
        <Button
            id
            icon
            on_click
            comp_ref
            appearance=ButtonAppearance::Transparent
            class=class_list!["!px-3", class]
        >
        </Button>
    }
}

#[component]
pub fn AddButton(
    /// Click handler
    #[prop(default = BoxOneCallback::new(|_| ()), into)]
    on_click: BoxOneCallback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <Button
            icon=icon::AddIcon()
            on_click=on_click
            appearance=ButtonAppearance::Primary
            class="m-2"
            {..}>
            Add
        </Button>
    }
}

#[component]
pub fn EditButton(
    /// Click handler
    #[prop(default = BoxOneCallback::new(|_| ()), into)]
    on_click: BoxOneCallback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <Button
            icon=icon::EditIcon()
            on_click=on_click
            appearance=ButtonAppearance::Secondary
            class="m-2"
            {..}>
            Edit
        </Button>
    }
}

#[component]
pub fn DeleteButton(
    /// Click handler
    #[prop(default = BoxOneCallback::new(|_| ()), into)]
    on_click: BoxOneCallback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <Button
            icon=icon::DeleteIcon()
            on_click=on_click
            appearance=ButtonAppearance::Danger
            class="m-2"
            {..}>
            Delete
        </Button>
    }
}

#[component]
pub fn DownloadButton(
    /// Click handler
    #[prop(default = BoxOneCallback::new(|_| ()), into)]
    on_click: BoxOneCallback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <Button
            icon=icon::DownloadIcon()
            on_click=on_click
            appearance=ButtonAppearance::Secondary
            class="m-2"
            {..}>
            Download
        </Button>
    }
}

impl From<&'static IconData> for MaybeProp<Signal<&'static IconData>> {
    fn from(value: &'static IconData) -> Self {
        MaybeProp::from(Signal::<&'static IconData>::from(value))
    }
}

#[slot]
pub struct DropdownButtonChildren {
    children: Children,
}

/// A button triggers an action or event when activated.
///
/// Example
/// ```
/// <DropdownButton>
///     <DropdownButtonChildren slot:button_children>DropDownButton</DropdownButtonChildren>
///     <DropdownItem label="Entry-1" on_click=move |e| { debug_log!("{e:?}"); } />
/// </DropdownButton>
/// ```
#[component]
pub fn DropdownButton(
    /// Button ID, dropdown ID is derived by "{button_id}-dropdown"
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Extra classes appened to the button's default style
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// How the dropdown aligns to its parent.
    #[prop(default = AlignmentAnchor::default(), into)]
    alignment: AlignmentAnchor,
    /// A button can have its content and borders styled for greater emphasis or to be subtle.
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// A button supports different sizes.
    #[prop(optional, into)]
    size: MaybeProp<Signal<ButtonSize>>,
    /// The default behavior of the button.
    #[prop(optional, into)]
    button_type: ButtonType,
    /// The shape of the button.
    #[prop(default = ButtonShape::default(), into)]
    shape: ButtonShape,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<Signal<IconRef>>,
    /// Whether the button shows the loading status.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Autoclose the dropdown upon clicking a dropdown item.
    #[prop(optional, default = true, into)]
    should_autoclose: bool,
    /// Most likely a label
    button_children: DropdownButtonChildren,
    /// Dropdown items
    /// e.g. <DropdownItem/>
    children: Children,
    /// comp_ref will be filled with a reference to the DOM element.
    #[prop(optional)]
    comp_ref: ComponentRef<ButtonRef>,
) -> impl IntoView
where
{
    let (is_visible, set_visible) = signal(false);

    let button = Button(ButtonProps {
        id,
        class: class.into(),
        appearance,
        size,
        button_type,
        shape,
        icon,
        loading,
        on_click: Some(BoxOneCallback::new(move |_e| {
            set_visible.update(|setter| *setter = !*setter);
        })),
        children: Some(button_children.children),
        comp_ref,
    });
    provide_context::<crate::dropdown::SetVisibleCallback>(set_visible);
    provide_context::<crate::dropdown::AutoClose>(should_autoclose);
    let dropdown_id = id.get().map(|id| format!("{id}-modal"));

    view! {
        <div class="fit-content relative">
            {button}
            <Dropdown id=dropdown_id is_visible alignment>
                {children().into_any()}
            </Dropdown>
        </div>
    }
}

#[slot]
pub struct ModalButtonChildren {
    children: Children,
}

/// A button to toggle a modal
#[component]
pub fn ModalButton(
    /// Button ID, modal ID is derived by "{button_id}-modal"
    #[prop(optional, into)]
    id: MaybeProp<String>,
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
    button_type: ButtonType,
    /// The shape of the button.
    #[prop(default = ButtonShape::default(), into)]
    shape: ButtonShape,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<Signal<IconRef>>,
    /// Whether the button shows the loading status.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Most likely a label
    button_children: ModalButtonChildren,
    /// comp_ref will be filled with a reference to the DOM element.
    #[prop(optional)]
    comp_ref: ComponentRef<ButtonRef>,

    /// Title shown in the modal heading
    #[prop(optional, into)]
    modal_title: MaybeProp<String>,
    /// True shows the modal, false hides it.
    #[prop(default = RwSignal::new(false), into)]
    modal_visible: RwSignal<bool>,
    /// Modal content
    children: Children,
    /// Modal footer (e.g. Ok and Cancel buttons)
    modal_footer: ModalFooterChildren,
) -> impl IntoView
where
{
    let button = Button(ButtonProps {
        id,
        class: class.into(),
        appearance,
        size,
        button_type,
        shape,
        icon,
        loading,
        on_click: Some(BoxOneCallback::new(move |_e| {
            modal_visible.update(|inner_visible| *inner_visible = !*inner_visible);
        })),
        children: Some(button_children.children),
        comp_ref,
    });
    let modal_id = id.get().map(|id| format!("{id}-modal"));

    view! {
        <div>
            {button}
            <Modal id=modal_id title=modal_title visible=modal_visible>
                {children()}
                <ModalFooterChildren slot:footer>
                    {(modal_footer.children)()}
                </ModalFooterChildren>
            </Modal>
        </div>
    }
}

#[slot]
pub struct DialogButtonChildren {
    children: Children,
}

/// A button to toggle a dialog
#[component]
pub fn DialogButton(
    /// Button ID, dialog ID is derived by "{button_id}-dialog"
    #[prop(optional, into)]
    id: MaybeProp<String>,
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
    button_type: ButtonType,
    /// The shape of the button.
    #[prop(default = ButtonShape::default(), into)]
    shape: ButtonShape,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<Signal<IconRef>>,
    /// Whether the button shows the loading status.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// comp_ref will be filled with a reference to the DOM element.
    #[prop(optional)]
    comp_ref: ComponentRef<ButtonRef>,
    /// Most likely a label
    button_children: DialogButtonChildren,
    /// Title shown in the dialog heading
    #[prop(optional, into)]
    dialog_title: MaybeProp<String>,
    /// True shows the dialog, false hides it.
    #[prop(default = RwSignal::new(false), into)]
    dialog_visible: RwSignal<bool>,
    /// Dialog primary-button
    #[prop(default = "Ok".into(), into)]
    primary_text: String,
    /// Click handler primary-button
    #[prop(default = BoxCallback::new(|| ()), into)]
    on_click_primary: BoxCallback,
    /// Dialog secondary-button
    #[prop(default = "Cancel".into(), into)]
    secondary_text: String,
    /// Click handler secondary-button
    #[prop(default = BoxCallback::new(|| ()), into)]
    on_click_secondary: BoxCallback,
    /// Dialog content
    children: Children,
) -> impl IntoView
where
{
    let button = Button(ButtonProps {
        id,
        class: class.into(),
        appearance,
        size,
        button_type,
        shape,
        icon,
        loading,
        on_click: Some(BoxOneCallback::new(move |_e| {
            dialog_visible.update(|inner_visible| *inner_visible = !*inner_visible);
        })),
        children: Some(button_children.children),
        comp_ref,
    });
    let dialog_id = id.get().map(|id| format!("{id}-dialog"));

    view! {
        <div>
            {button}
            <Dialog id=dialog_id title=dialog_title visible=dialog_visible primary_text secondary_text on_click_primary on_click_secondary>
                {children()}
            </Dialog>
        </div>
    }
}
