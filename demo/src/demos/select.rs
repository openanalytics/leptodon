use attr_docgen::generate_codeblock;
use derive_more::Display;
use leptos::oco::Oco;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use leptos_components::button::Button;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_components::radio::RadioOption;
use leptos_components::select::MaybeSelect;
use leptos_components::select::Select;
use leptos_meta::Title;

#[generate_codeblock(MaybeSelectExample)]
#[component]
pub fn MaybeSelectDemo() -> impl IntoView {
    #[derive(Display, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    enum SelectStation {
        Select1,
        Select2,
        Klara,
    }
    impl RadioOption for SelectStation {
        fn value(&self) -> Oco<'static, str> {
            match self {
                SelectStation::Select1 => "select_1",
                SelectStation::Select2 => "select_2",
                SelectStation::Klara => "klara",
            }
            .into()
        }
    }
    let options = RwSignal::new(vec![
        SelectStation::Select1,
        SelectStation::Select2,
        SelectStation::Klara,
    ]);
    let selected = RwSignal::new(None);
    view! {
        <p>"Selected option: "{move || format!("{:?}", selected.get())}</p>
        <MaybeSelect
            name="select_station"
            label="Select Stations"
            selected
            options
            required=true
        />
        <Button on_click=move |_| {
            selected.set(Some(SelectStation::Klara));
        }>
            "Set Klara as selected"
        </Button>
    }
}

#[derive(Display, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum SelectStation {
    Select1,
    Select2,
    Klara,
}
impl RadioOption for SelectStation {
    fn value(&self) -> Oco<'static, str> {
        match self {
            SelectStation::Select1 => "select_1",
            SelectStation::Select2 => "select_2",
            SelectStation::Klara => "klara",
        }
        .into()
    }
}

#[generate_codeblock(SelectExample)]
#[component]
pub fn SelectDemo() -> impl IntoView {
    let options = RwSignal::new(vec![
        SelectStation::Select1,
        SelectStation::Select2,
        SelectStation::Klara,
    ]);
    let selected = RwSignal::new(SelectStation::Select1);
    view! {
        <p>"Selected option: "{move || format!("{:?}", selected.get())}</p>
        <Select
            name="select_station"
            label="Select Stations"
            selected
            options
            required=true
        />
        <Button on_click=move |_| {
            selected.set(SelectStation::Klara);
        }>
            "Set Klara as selected"
        </Button>
    }
}

#[component]
pub fn SelectDemoPage() -> impl IntoView {
    view! {
        <Title text="Select"/>

        <FixedCenterColumn>
            <Heading4 anchor="maybe-select">"MaybeSelect"</Heading4>
            <p>"The MaybeSelect<T> needs an Option<T> value."</p>
            <MaybeSelectExample />

            <Heading4 anchor="select">"Select"</Heading4>
            <p>"The Select<T> needs a T value."</p>
            <SelectExample />

            <leptos_components::select::MaybeSelectDocs />
            <leptos_components::select::SelectDocs />
        </FixedCenterColumn>
    }
}
