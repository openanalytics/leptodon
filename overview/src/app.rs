use leptos_components::input_group::InputGroup;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_components::button::DropdownButton;
use leptos_components::button::DropdownButtonChildren;
use leptos_components::darkmode::ThemeSelector;
use leptos_components::dropdown::DropdownItem;
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
            <ControlledNumberInput/>
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
pub fn ControlledNumberInput() -> impl IntoView {
    view! {
        <InputGroup>
            <Input name="number" placeholder="5" input_type=InputType::Number />
            <Button>-</Button>
            <Button icon=leptos_components::icon::AddIcon() />
        </InputGroup>

        <form class="max-w-xs mx-auto">
            <label for="counter-input" class="block mb-1 text-sm font-medium text-gray-900 dark:text-white">Choose quantity:</label>
            <div class="relative flex items-center">
                <button type="button" id="decrement-button" data-input-counter-decrement="counter-input" class="flex-shrink-0 bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 inline-flex items-center justify-center border border-gray-300 rounded-md h-5 w-5 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-2.5 h-2.5 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 2">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h16"/>
                    </svg>
                </button>
                <input type="text" id="counter-input" data-input-counter class="flex-shrink-0 text-gray-900 dark:text-white border-0 bg-transparent text-sm font-normal focus:outline-none focus:ring-0 max-w-[2.5rem] text-center" placeholder="" value="12" required />
                <button type="button" id="increment-button" data-input-counter-increment="counter-input" class="flex-shrink-0 bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 inline-flex items-center justify-center border border-gray-300 rounded-md h-5 w-5 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-2.5 h-2.5 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 1v16M1 9h16"/>
                    </svg>
                </button>
            </div>
        </form>

        <form class="max-w-xs mx-auto">
            <label for="bedrooms-input" class="sr-only">Choose bedrooms number:</label>
            <div class="relative flex items-center mb-2">
                <button type="button" id="decrement-button" data-input-counter-decrement="bedrooms-input" class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-s-lg p-3 h-11 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 2">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h16"/>
                    </svg>
                </button>
                <input type="text" id="bedrooms-input" data-input-counter data-input-counter-min="1" data-input-counter-max="5" class="bg-gray-50 border-x-0 border-gray-300 h-11 font-medium text-center text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full pb-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="" value="2" required />
                <div class="absolute bottom-1 start-1/2 -translate-x-1/2 rtl:translate-x-1/2 flex items-center text-xs text-gray-400 space-x-1 rtl:space-x-reverse">
                    <svg class="w-2.5 h-2.5 text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8v10a1 1 0 0 0 1 1h4v-5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5h4a1 1 0 0 0 1-1V8M1 10l9-9 9 9"/>
                    </svg>
                    <span>Bedrooms</span>
                </div>
                <button type="button" id="increment-button" data-input-counter-increment="bedrooms-input" class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-e-lg p-3 h-11 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 1v16M1 9h16"/>
                    </svg>
                </button>
            </div>
            <label for="nights-input" class="sr-only">Choose number of nights:</label>
            <div class="relative flex items-center mb-2">
                <input type="text" id="nights-input" data-input-counter data-input-counter-min="1" data-input-counter-max="30" class="bg-gray-50 border-gray-300 h-11 font-medium text-center text-gray-900 text-sm rounded-s-lg focus:ring-blue-500 focus:border-blue-500 block w-full dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="" value="7" required />
                <button type="button" id="decrement-button" data-input-counter-decrement="nights-input" class="bg-gray-100 dark:bg-gray-700 border-x-0 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 p-3 h-11 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 2">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h16"/>
                    </svg>
                </button>
                <button type="button" id="increment-button" data-input-counter-increment="nights-input" class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-e-lg p-3 h-11 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 1v16M1 9h16"/>
                    </svg>
                </button>
            </div>
            <div class="relative flex items-center mb-2">
                <button type="button" id="decrement-button" data-input-counter-decrement="nights-input" class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-s-lg p-3 h-11 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 2">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h16"/>
                    </svg>
                </button>
                <input type="text" id="nights-input" data-input-counter data-input-counter-min="1" data-input-counter-max="30" class="bg-gray-50 border-x-0 border-gray-300 h-11 font-medium text-center text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="" value="7" required />
                <button type="button" id="increment-button" data-input-counter-increment="nights-input" class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-e-lg p-3 h-11 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 1v16M1 9h16"/>
                    </svg>
                </button>
            </div>
            <label for="guests-input" class="sr-only">Choose guests:</label>
            <div class="relative flex items-center">
                <button type="button" id="decrement-button" data-input-counter-decrement="guests-input" class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-s-lg p-3 h-11 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 2">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h16"/>
                    </svg>
                </button>
                <input type="text" id="guests-input" data-input-counter data-input-counter-min="1" data-input-counter-max="5" class="bg-gray-50 border-x-0 border-gray-300 h-11 font-medium text-center text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full pb-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="" value="3" required />
                <div class="absolute bottom-1 start-1/2 -translate-x-1/2 rtl:translate-x-1/2 flex items-center text-xs text-gray-400 space-x-1 rtl:space-x-reverse">
                    <svg class="w-2.5 h-2.5 text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.333 6.764a3 3 0 1 1 3.141-5.023M2.5 16H1v-2a4 4 0 0 1 4-4m7.379-8.121a3 3 0 1 1 2.976 5M15 10a4 4 0 0 1 4 4v2h-1.761M13 7a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm-4 6h2a4 4 0 0 1 4 4v2H5v-2a4 4 0 0 1 4-4Z"/>
                    </svg>
                    <span>Guests</span>
                </div>
                <button type="button" id="increment-button" data-input-counter-increment="guests-input" class="bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 border border-gray-300 rounded-e-lg p-3 h-11 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 1v16M1 9h16"/>
                    </svg>
                </button>
            </div>
        </form>

        <form class="max-w-xs mx-auto">
            <label for="counter-input" class="block mb-1 text-sm font-medium text-gray-900 dark:text-white">Choose quantity:</label>
            <div class="relative flex items-center">
                <button type="button" id="decrement-button" data-input-counter-decrement="counter-input" class="flex-shrink-0 bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 inline-flex items-center justify-center border border-gray-300 rounded-md h-5 w-5 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-2.5 h-2.5 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 2">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h16"/>
                    </svg>
                </button>
                <input type="text" id="counter-input" data-input-counter class="flex-shrink-0 text-gray-900 dark:text-white border-0 bg-transparent text-sm font-normal focus:outline-none focus:ring-0 max-w-[2.5rem] text-center" placeholder="" value="12" required />
                <button type="button" id="increment-button" data-input-counter-increment="counter-input" class="flex-shrink-0 bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:border-gray-600 hover:bg-gray-200 inline-flex items-center justify-center border border-gray-300 rounded-md h-5 w-5 focus:ring-gray-100 dark:focus:ring-gray-700 focus:ring-2 focus:outline-none">
                    <svg class="w-2.5 h-2.5 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 1v16M1 9h16"/>
                    </svg>
                </button>
            </div>
        </form>

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
