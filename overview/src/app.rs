use crate::group_table::GroupedTableExample;
use crate::web_calendar::PopulatedCalendar;
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
use leptos_components::button::ModalButton;
use leptos_components::button::ModalButtonChildren;
use leptos_components::button_group::ButtonGroup;
use leptos_components::button_group::First;
use leptos_components::button_group::Last;
use leptos_components::checkbox::Checkbox;
use leptos_components::darkmode::ThemeSelector;
use leptos_components::date_picker::DatePicker;
use leptos_components::date_picker::range_picker::DateRangePicker;
use leptos_components::dropdown::DropdownItem;
use leptos_components::input::InputType;
use leptos_components::link::Link;
use leptos_components::modal::ModalFooterChildren;
use leptos_components::navbar::SideNavbar;
use leptos_components::table::DemoTable;
use leptos_components::toggle::Toggle;
use leptos_components::{
    button::{Button, ButtonAppearance},
    input::Input,
};
use leptos_meta::MetaTags;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_meta::provide_meta_context;
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
    let (count, set_count) = signal(1);
    let double_count = move || count.get() * 2;
    let modal_visible = RwSignal::new(false);

    view! {
        <Title text="Leptos components" />
        <SideNavbar>
        <main class="px-3">
            <Settings>
                <ThemeSelector />
            </Settings>
            <GroupedTableExample />
            <DemoTable />
            <PopulatedCalendar />
            <Link href="https://www.openanalytics.eu/">"Click here"</Link>
            <DropdownButton {..} data-testid="test-dropdown">
                <DropdownButtonChildren slot:button_children>DropDownButton</DropdownButtonChildren>
                // <li>hi</li>
                <DropdownItem label="Entry-1" on_click=move |e| { log!("{:?}", e); } {..} data-testid="test-dropdown-item1" />
            </DropdownButton>
            <ModalButton id="modal-button1" modal_title="Example modal" modal_visible>
                <ModalButtonChildren slot:button_children>Toggle Modal</ModalButtonChildren>

                <p class="leading-relaxed text-body">
                    Adipisci deserunt officia omnis. Et velit et consequatur sed porro minus unde expedita. Similique distinctio dolorem sunt voluptate laboriosam aut autem.
                </p>
                <p class="leading-relaxed text-body">
                    Officiis qui id delectus quia sunt quisquam voluptatem modi. Velit iste quia asperiores alias. Modi aut enim nostrum nihil laudantium sit perferendis delectus. Voluptatem repudiandae culpa doloribus aut. Cupiditate quisquam iusto illum quo rem cum. Vitae soluta est pariatur ut.
                </p>

                <ModalFooterChildren slot:modal_footer>
                    <Button on_click=move |_| modal_visible.set(false)>"Dispose modal"</Button>
                </ModalFooterChildren>
            </ModalButton>
            <div>
                <leptos_components::input_group::ControlledNumberInput<i32> class="w-96" min=-1 max=15 />
            </div>
            {
                view! {
                    <DatePicker id="date_picker" value=RwSignal::new(None) />
                    <DateRangePicker id="date_range_picker" />
                }.into_any()
            }
            <br/>
            <Toggle label="Lightswitch" value=Signal::derive(move || false) />
            <Avatar src="/favicon.ico" />
            <Avatar />
            <AnyAccordion />

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
                on_click=move |_| { set_count.set(count.get()*2) }
                appearance=ButtonAppearance::Primary
                class="m-2"
                {..}
                id="5">
                {move || count.get()*2}
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
            <Checkbox label="Done all tasks" value=Signal::derive(|| false)/>
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
        </SideNavbar>
    }
}

#[component]
pub fn AnyAccordion() -> AnyView {
    view!{
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
    }.into_any()
}

#[component]
pub fn Settings(children: Children) -> AnyView {
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
    }.into_any()
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
