use leptos::prelude::*;
use leptos_components::input::InputType;

use leptos_components::{
    button::{Button, ButtonAppearance},
    input::Input,
};
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <Title text="Leptos components" />
        <main class="px-3">
            <button
                on:click=move |_| { *set_count.write() += 1 }
                class=("red", move || count.get() % 2 == 1)
                style="position: absolute"
                style:left=move || format!("{}px", count.get() + 100)
            >
                "Click me: "
                {count}
            </button>
            <p>"Double count: " {double_count}</p>
            <ProgressBar label="shark" progress=Signal::derive(double_count) />
            <ProgressBar progress=count />

            <Button
                on_click=move |_| {}
                appearance=ButtonAppearance::Primary
                class="m-2"
                {..}
                id="5">
                5
            </Button>
            <br/>
            <Button
                icon=leptos_components::icon::AddIcon()
                on_click=move |_| {}
                appearance=ButtonAppearance::Secondary
                class="m-2"
                {..}
                id="add">
                Add
            </Button>
            <br/>
            <Input/>
            <br/>
            <Input label="Username" name="username" placeholder="Username" input_type=InputType::Text  />
            <Input label="Email" name="email" placeholder="Email" input_type=InputType::Email readonly=true />
            <Input label="Password" name="pass" placeholder="Password" input_type=InputType::Password />
            <br/>
            <crate::gen_icons::IconList />
        </main>
    }
}

// An html progress bar component
#[component]
fn ProgressBar(
    #[prop(into, default="fish".to_string())] label: String,
    // Updatable value of the progress bar
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <br />
        <progress
            max="50"
            // hmm... where will we get this from?
            value=progress
        />
        <p>{label}</p>
    }
}
