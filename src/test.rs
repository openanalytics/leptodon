use leptos::prelude::ClassAttribute;
use leptos::prelude::Get;
use leptos::prelude::OnAttribute;
use leptos::prelude::{ElementChild, Update, signal};
use leptos::{IntoView, component, view};

#[component]
pub fn TestPage() -> impl IntoView {
    let (value, set_value) = signal(0);
    view! {
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
            <div class="flex flex-row-reverse flex-wrap m-auto">
                <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                    "+"
                </button>
                <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                    {value}
                </button>
                <button
                    on:click=move |_| set_value.update(|value| *value -= 1)
                    class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                    class:invisible=move || {value.get() < 1}
                >
                    "-"
                </button>
                <button type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">Default</button>
            </div>
        </div>
    }
}
