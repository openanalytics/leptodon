use crate::button::ButtonProps;
use crate::button::ButtonShape;
use crate::button::{Button, ButtonAppearance, ButtonRef, ButtonSize, ButtonType};
use crate::button_group::ButtonGroup;
use crate::button_group::First;
use crate::button_group::Last;
use crate::class_list;
use crate::dropdown::AlignmentAnchor;
use crate::dropdown::Dropdown;
use crate::icon;
use crate::icon::icon_data::IconData;
use crate::icon::icon_data::IconRef;
use crate::input::GenericInput;
use crate::input::Input;
use crate::input::InputMode;
use crate::input::InputType;
use crate::modal::{Modal, ModalFooterChildren};
use crate::util::callback::BoxOneCallback;
use crate::util::signals::ComponentRef;
use crate::util::signals::Model;
use leptos::prelude::AriaAttributes;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::IntoAny;
use leptos::prelude::Set;
use leptos::prelude::{Children, Get, MaybeProp, Signal, provide_context, signal};
use leptos::prelude::{ElementChild, RwSignal, Update};
use leptos::{IntoView, component, view};
use leptos::{ev, slot};

/// Shows a `<< < 1 ... 2 ... 32 > >>`-like button group, allowing for user navigation of paged content.
#[component]
pub fn Pagination(
    /// Total number of pages
    page_count: Signal<usize>,
    /// Updated with the desired target page upon a user pagination action (e.g. next, prev, jump)
    /// Target page is clamped to (1..*nb_pages*).
    /// Read for intial page position
    current_page: RwSignal<usize>,
    jumper: bool,
) -> impl IntoView {
    let read_only = Signal::derive(move || !jumper);
    let last_icon = crate::icon::LastIcon();
    let first_icon = crate::icon::FirstIcon();
    let prev_icon = crate::icon::PreviousIcon();
    let next_icon = crate::icon::NextIcon();
    let appearance = ButtonAppearance::Secondary;
    view! {
        <nav aria-label="Page navigation example">
            <ButtonGroup>
                <First slot:first>
                    <Button class="mr-0" on_click=move |_| { current_page.set(1); } appearance icon=first_icon></Button>
                </First>
                <Button on_click=move |_| { current_page.update(|old| *old = std::cmp::max(*old-1, 1)); } appearance icon=prev_icon></Button>
                <GenericInput<usize, String> name="pagination page" input_mode=InputMode::Numeric input_type=InputType::Number value=current_page readonly=read_only/>
                <Button on_click=move |_| { current_page.update(|old| *old = std::cmp::min(*old+1, page_count.get())); } appearance icon=next_icon></Button>
                <Last slot:last>
                    <Button on_click=move |_| { current_page.set(page_count.get()); } appearance icon=last_icon></Button>
                </Last>
            </ButtonGroup>
        </nav>
    }
}
