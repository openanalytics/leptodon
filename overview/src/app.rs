use leptos::logging::log;
use leptos::prelude::*;
use leptos_components::button::DropdownButton;
use leptos_components::button::DropdownButtonChildren;
use leptos_components::darkmode::ThemeSelector;
use leptos_components::dropdown::DropdownItem;
use leptos_components::input::InputType;
use leptos_components::input_group::InputGroup;

use leptos_components::{
    button::{Button, ButtonAppearance},
    input::Input,
};
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Stylesheet href="/pkg/overview.css"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </Routes>
        </Router>
    }
}

// #[component]
// fn Home() -> impl IntoView {
//     view! {
//         <p>Hello World!</p>
//     }
// }

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    // let initial = initial_prefers_dark();

    // let toggle_dark_mode_action = ServerAction::<UpdateTheme>::new();
    // // input is `Some(value)` when pending, and `None` if not pending
    // let input = toggle_dark_mode_action.input();
    // // value contains most recently-returned value
    // let value = toggle_dark_mode_action.value();

    // let prefers_dark = move || {
    //     match (input.get(), value.get()) {
    //         // if there's some current input, use that optimistically
    //         (Some(submission), _) => submission.prefers_dark,
    //         // otherwise, if there was a previous value confirmed by server, use that
    //         (_, Some(Ok(value))) => value,
    //         // otherwise, use the initial value
    //         _ => initial,
    //     }
    // };

    // let style = "selection:bg-oa-blue selection:text-white";

    // let color_scheme = move || {
    //     if prefers_dark() {
    //         format!("dark {style}")
    //     } else {
    //         format!("light {style}")
    //     }
    // };

    view! {

        <Meta
            name="color-scheme"
            content="light dark"
        />
        <Title text="Leptos components" />
        <main class="px-3">
            <Settings>
                <ThemeSelector />
            </Settings>
            <DropdownButton>
                <DropdownButtonChildren slot:button_children>DropDownButton</DropdownButtonChildren>
                // <li>hi</li>
                <DropdownItem label="Entry-1" on_click=move |e| { log!("{:?}", e); }/>
            </DropdownButton>
            <div>
                <leptos_components::input_group::ControlledNumberInput<i32> class="w-96" min=-1 max=15 />
            </div>
            <br/>
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

#[component]
pub fn Settings(children: Children) -> impl IntoView {
    view! {
        <section class="bg-white dark:bg-gray-900">
            <div class="grid max-w-screen-xl px-4 py-4 mx-auto lg:gap-8 xl:gap-0">
                <div class="mr-auto">
                    <h2
                        id="settings-title"
                        class="max-w-2xl mb-4 text-xl font-medium tracking-tight leading-none md:text-2xl xl:text-3xl dark:text-white"
                    >
                        "Settings"
                    </h2>
                    <div class="max-w-2xl mb-6 lg:mb-8">{children()}</div>
                </div>
            </div>
        </section>
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
