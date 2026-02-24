use attr_docgen::generate_codeblock;
use derive_more::Display;
use leptodon::button::Button;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::radio::Radio;
use leptodon::radio::RadioOption;
use leptos::oco::Oco;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(RadioExample)]
#[component]
pub fn RadioDemo() -> impl IntoView {
    #[derive(Display, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    enum RadioStation {
        Radio1,
        Radio2,
        Klara,
    }
    impl RadioOption for RadioStation {
        fn value(&self) -> Oco<'static, str> {
            match self {
                RadioStation::Radio1 => "radio_1",
                RadioStation::Radio2 => "radio_2",
                RadioStation::Klara => "klara",
            }
            .into()
        }
    }
    let radio_options = RwSignal::new(vec![
        RadioStation::Radio1,
        RadioStation::Radio2,
        RadioStation::Klara,
    ]);
    let selected = RwSignal::new(None);
    view! {
        <p>"Selected option: "{move || format!("{:?}", selected.get())}</p>
        <Radio
            name="radio_station"
            label="Radio Stations"
            options=radio_options
            selected
            required=true
        />
        <Button on_click=move |_| {
            selected.set(Some(RadioStation::Klara));
        }>
            "Set Klara as selected"
        </Button>
    }
}

#[component]
pub fn RadioDemoPage() -> impl IntoView {
    view! {
        <Title text="Radio"/>

        <FixedCenterColumn>
            <Heading4 anchor="radio">"Radio Buttons"</Heading4>
            <RadioExample />

            <leptodon::radio::RadioDocs />
        </FixedCenterColumn>
    }
}
