mod components;

use leptos::html::Button;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    view! {
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
        <ProgressBar progress=Signal::derive(double_count) />
        <ProgressBar progress=count />
        <Button id="hi" label="test" on_click=move || {} />
    }
}

// An html progress bar component
#[component]
fn ProgressBar(
    // Updatable value of the progress bar
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <br />
        <progress
            max="50"
            // hmm... where will we get this from?
            value=progress
        />
    }
}