use crate::button::{Button, ButtonAppearance};
use crate::button_group::ButtonGroup;
use crate::button_group::First;
use crate::button_group::Last;
use crate::input::GenericInput;
use crate::input::InputMode;
use crate::input::InputType;
use crate::util::callback::ArcOneCallback;
use crate::util::callback::BoxOneCallback;
use leptos::prelude::Set;
use leptos::prelude::{AriaAttributes, IntoAny};
use leptos::prelude::{ElementChild, RwSignal, Update};
use leptos::prelude::{Get, Signal};
use leptos::{IntoView, component, view};

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
    let last_icon = crate::icon::LastIcon();
    let first_icon = crate::icon::FirstIcon();
    let prev_icon = crate::icon::PreviousIcon();
    let next_icon = crate::icon::NextIcon();
    let appearance = ButtonAppearance::Secondary;
    let format = BoxOneCallback::new(|s: usize| format!("{s}"));
    let parser = ArcOneCallback::new(|string: String| {
        string.parse::<usize>().map_err(|err| {
            format!("Could not parse input to a positive number because of: {err:?}")
        })
    });
    view! {
        <nav aria-label="Page navigation example">
            <ButtonGroup>
                <First slot:first>
                    <Button class="mr-0" on_click=move |_| { current_page.set(1); } appearance icon=first_icon></Button>
                </First>
                <Button appearance icon=prev_icon on_click=move |_| {
                    current_page.update(|old| *old = std::cmp::max(*old-1, 1));
                }></Button>
                
                {if jumper {
                    view! {
                        <GenericInput<usize, String>
                            name="pagination page"
                            input_mode=InputMode::Numeric
                            input_type=InputType::Number
                            value=current_page
                            format
                            parser
                        />
                    }.into_any()
                } else {
                    ().into_any()
                }}

                <Button appearance icon=next_icon on_click=move |_| {
                    current_page.update(|old| *old = std::cmp::min(*old+1, page_count.get()));
                } ></Button>
                <Last slot:last>
                    <Button on_click=move |_| { current_page.set(page_count.get()); } appearance icon=last_icon></Button>
                </Last>
            </ButtonGroup>
        </nav>
    }
}
