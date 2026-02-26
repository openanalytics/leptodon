// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
use crate::demo_table::DemoTable;
use crate::group_table::GroupedTableExample;
use crate::web_calendar::PopulatedCalendar;
use chrono::NaiveDate;
use leptodon::accordion::Accordion;
use leptodon::accordion::AccordionEntry;
use leptodon::avatar::Avatar;
use leptodon::button::AddButton;
use leptodon::button::DeleteButton;
use leptodon::button::DialogButton;
use leptodon::button::DialogButtonChildren;
use leptodon::button::DownloadButton;
use leptodon::button::DropdownButton;
use leptodon::button::DropdownButtonChildren;
use leptodon::button::EditButton;
use leptodon::button::ModalButton;
use leptodon::button::ModalButtonChildren;
use leptodon::button_group::ButtonGroup;
use leptodon::button_group::First;
use leptodon::button_group::Last;
use leptodon::checkbox::Checkbox;
use leptodon::codeblock::Codeblock;
use leptodon::darkmode::ThemeSelector;
use leptodon::date_picker::DatePicker;
use leptodon::date_picker::range_picker::DateRangePicker;
use leptodon::dropdown::DropdownItem;
use leptodon::heading::*;
use leptodon::icon;
use leptodon::input::FileUpload;
use leptodon::input::InputType;
use leptodon::link::Link;
use leptodon::modal::ModalFooterChildren;
use leptodon::navbar::NavbarEntries;
use leptodon::navbar::SideBarLink;
use leptodon::navbar::SideNavbar;
use leptodon::spinner::Spinner;
use leptodon::textarea::TextArea;
use leptodon::toggle::Toggle;
use leptodon::{
    button::{Button, ButtonAppearance},
    input::TextInput,
};
use leptos::logging::log;
use leptos::prelude::*;
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
                <link rel="shortcut icon" type_="image/svg+xml" href="/logo.svg"/>

                // Metadata injection is not allowed here, only use them in components down the chain
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="dark:bg-[#030712]">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet href="/pkg/overview.css"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
                <Route path=StaticSegment("/test_select") view=crate::testcases::select::TestSelect/>
                <Route path=StaticSegment("/test_radio") view=crate::testcases::radio::TestRadio/>
                <Route path=StaticSegment("/test_tag_picker") view=crate::testcases::tag_picker::TestTagPicker/>
                <Route path=StaticSegment("/test_inputs") view=crate::testcases::inputs::TestInputs/>
                <Route path=StaticSegment("/test_toggle") view=crate::testcases::toggle::TestToggle/>
                <Route path=StaticSegment("/test_checkbox") view=crate::testcases::checkbox::TestCheckbox/>
                <Route path=StaticSegment("/test_popover") view=crate::testcases::popover::TestPopover/>
                <Route path=StaticSegment("/test_calendar") view=crate::testcases::calendar::TestCalendar/>
                <Route path=StaticSegment("/test_copy_button") view=crate::testcases::copy_button::TestCopyButton/>
                <Route path=StaticSegment("/test_upload") view=crate::testcases::upload::TestUpload/>
                <Route path=StaticSegment("/forms") view=crate::forms::Forms/>
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
    let dialog_visible = RwSignal::new(false);
    let text_area_input = RwSignal::new("Hi,\nMultiline".to_string());
    view! {
        <Title text="Leptodon" />
        <SideNavbar>
            <NavbarEntries slot:entries>
                <li><SideBarLink href="#" icon=icon::CalendarIcon()>Calendar</SideBarLink></li>
                <li><SideBarLink href="users" icon=icon::UserIcon()>Users</SideBarLink></li>
                <li><SideBarLink href="forms" icon=icon::PendingApprovalIcon()>Users</SideBarLink></li>
                <li><SideBarLink href="security" icon=icon::LockOpenIcon()>Security</SideBarLink></li>
            </NavbarEntries>
        <main class="px-3">
            <Settings>
                <ThemeSelector />
            </Settings>
            <FileUpload multiple=true accept="image/png" />
            <Spinner />
            <Heading1 anchor="the-largest-heading">The Largest Heading</Heading1>
            <Heading2 class="color-red-500">The 2nd Largest Heading</Heading2>
            <Heading3>The Large Heading</Heading3>
            <Heading4>The Heading</Heading4>
            <Heading5>The Smaller Heading</Heading5>
            <Heading6>The Smallest Heading</Heading6>
            <Codeblock code="impl RadioOption for Element {
    fn value(&self) -> Oco<'static, str> {
        AsRef::<str>::as_ref(&self).to_string().into()
    }
}">
            </Codeblock>
            <TextArea value=text_area_input label="Notes" placeholder="Time for text" />
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
            <DialogButton id="dialog-button1" dialog_title="Example dialog" dialog_visible>
                <DialogButtonChildren slot:button_children>Toggle Dialog</DialogButtonChildren>

                <p class="leading-relaxed text-body">
                    Adipisci deserunt officia omnis. Et velit et consequatur sed porro minus unde expedita. Similique distinctio dolorem sunt voluptate laboriosam aut autem.
                </p>
                <p class="leading-relaxed text-body">
                    Officiis qui id delectus quia sunt quisquam voluptatem modi. Velit iste quia asperiores alias. Modi aut enim nostrum nihil laudantium sit perferendis delectus. Voluptatem repudiandae culpa doloribus aut. Cupiditate quisquam iusto illum quo rem cum. Vitae soluta est pariatur ut.
                </p>
            </DialogButton>
            <div>
                <leptodon::input_group::ControlledNumberInput<i32> id="controlled_number_input" class="w-96" min=-1 max=15 />
            </div>
            {
                view! {
                    <DatePicker id="date_picker" value=RwSignal::new(None) />
                    <DateRangePicker id="date_range_picker" />
                    <DateRangePicker id="date_range_picker_min_max"
                        min_date=NaiveDate::from_ymd_opt(2020, 10, 10).unwrap()
                        max_date=NaiveDate::from_ymd_opt(2030, 10, 10).unwrap() />
                }.into_any()
            }
            <br/>
            <Toggle checked=RwSignal::new(false)>"Lightswitch"</Toggle>
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
            <Checkbox checked=RwSignal::new(false)>
                Done all tasks
            </Checkbox>
            <br/>
            <AddButton/>
            <EditButton/>
            <DeleteButton/>
            <DownloadButton/>
            <br/>
            <TextInput/>
            <br/>
            <TextInput label="Username" name="username" placeholder="Username" input_type=InputType::Text  />
            <TextInput label="Email" name="email" placeholder="Email" input_type=InputType::Email readonly=true />
            <TextInput label="Password" name="pass" placeholder="Password" input_type=InputType::Password />
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
                <p class="mb-2 text-body">"Check out this guide to learn how to "<Link href="/docs/getting-started/introduction/">get started</Link>" and start developing websites even faster with components on top of Tailwind
                CSS."</p>

                <Accordion>
                    <AccordionEntry title="What about version 2.7.2?">
                        <p class="mb-2 text-body">"Version 2.7.2 is available at "<Link href="https://web.archive.org/web/20240328025144/https://flowbite.com/docs/components/dropdowns/">this location</Link></p>
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
