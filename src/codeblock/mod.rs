use std::time::Duration;

use crate::button::{Button, ButtonAppearance};
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::ReadValue;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::StoredValue;
use leptos::prelude::set_timeout;
use leptos::*;
use web_sys::window;

/// Styled monospace-rectangle for showing code, copy button in the top right
#[component]
pub fn Codeblock(
    /// The code to show in the block.
    #[prop(into)]
    code: String,
) -> impl IntoView {
    let stored_code = StoredValue::new(code);
    let btn_text = RwSignal::new("Copy");
    let on_copy = move |_| {
        let _ = window()
            .expect("Window should be present")
            .navigator()
            .clipboard()
            .write_text(&stored_code.read_value());
        btn_text.set("Copied!");
        set_timeout(
            move || {
                btn_text.set("Copy");
            },
            Duration::from_secs(2),
        );
    };

    view! {
        <div class="bg-codeblock-light dark:bg-codeblock-dark rounded-lg p-4 relative shadow-sm">
            <Button
                appearance=ButtonAppearance::Transparent
                class="absolute top-2 right-2 !mr-0 bg-white dark:bg-black"
                on_click=on_copy
            >
                {btn_text}
            </Button>
            <pre class="overflow-x-auto cursor-text text-wrap">{ stored_code.read_value().clone() }</pre>
        </div>
    }
}
