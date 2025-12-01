use leptos::logging::log;
use leptos::prelude::*;
use leptos_components::accordion::Accordion;
use leptos_components::accordion::AccordionEntry;
use leptos_components::avatar::Avatar;
use leptos_components::button::AddButton;
use leptos_components::button::DeleteButton;
use leptos_components::button::DownloadButton;
use leptos_components::button::DropdownButton;
use leptos_components::button::DropdownButtonChildren;
use leptos_components::button::EditButton;
use leptos_components::button_group::ButtonGroup;
use leptos_components::button_group::First;
use leptos_components::button_group::Last;
use leptos_components::darkmode::ThemeSelector;
use leptos_components::dropdown::DropdownItem;
use leptos_components::input::InputType;
use leptos_components::input_group::InputGroup;
use leptos_components::toggle::Toggle;
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
                // Metadata injection is not allowed here, only use them in components down the chain
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
        <Stylesheet href="/pkg/overview.css"/>
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

    view! {
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
            <Toggle label="Lightswitch" value=Signal::derive(move || false) />
            <Avatar src="/favicon.ico" />
            <Avatar />
            <Accordion>
                <AccordionEntry title="What is Flowbite?">
                    <p class="mb-2 text-body">Flowbite is an open - source library of interactive components built on top of
                    Tailwind CSS including buttons, dropdowns, modals, navbars, and more.</p>
                    <p class="mb-2 text-body">Check out this guide to learn how to<a href="/docs/getting-started/introduction/" class="text-fg-brand hover:underline">get started</a>and start developing websites even faster with components on top of Tailwind
                    CSS.</p>

                    <Accordion>
                        <AccordionEntry title="What about version 2.7.2?">
                            <p class="mb-2 text-body">Version 2.7.2 is available at <a href="https://web.archive.org/web/20240328025144/https://flowbite.com/docs/components/dropdowns/">this location</a></p>
                        </AccordionEntry>
                    </Accordion>
                </AccordionEntry>
            </Accordion>

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
            <ButtonGroup>
                <First slot:first>
                    <Button class="mr-0" on_click=move |_| {} appearance=ButtonAppearance::Secondary>Profile</Button>
                </First>
                <Button on_click=move |_| {} appearance=ButtonAppearance::Secondary>Settings</Button>
                <Last slot:last>
                    <Button on_click=move |_| {} appearance=ButtonAppearance::Secondary>Messages</Button>
                </Last>
            </ButtonGroup>
            <br/>
            <ButtonGroup>
                <First slot:first>
                    <Button class="mr-0" on_click=move |_| {} appearance=ButtonAppearance::Secondary>Profile</Button>
                </First>
                <Last slot:last>
                    <Button on_click=move |_| {} appearance=ButtonAppearance::Secondary>Messages</Button>
                </Last>
            </ButtonGroup>
            <br/>
            <ButtonGroup>
                <First slot:first>
                    <Button class="mr-0" on_click=move |_| {} appearance=ButtonAppearance::Secondary>Profile</Button>
                </First>
                <Button on_click=move |_| {} appearance=ButtonAppearance::Secondary>Settings</Button>
                <Button on_click=move |_| {} appearance=ButtonAppearance::Secondary>Settings2</Button>
                <Last slot:last>
                    <Button on_click=move |_| {} appearance=ButtonAppearance::Secondary>Messages</Button>
                </Last>
            </ButtonGroup>
            <br/>
            <AddButton/>
            <EditButton/>
            <DeleteButton/>
            <DownloadButton/>
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
