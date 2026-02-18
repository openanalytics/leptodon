use crate::button::CopyButton;
use attr_docgen::generate_docs;
use leptos::prelude::ElementChild;
use leptos::prelude::{ClassAttribute, Signal};
use leptos::{IntoView, component, view};

#[generate_docs]
/// Styled monospace-rectangle for showing code, copy button in the top right
#[component]
pub fn Codeblock(
    /// The code to show in the block.
    #[prop(into)]
    code: String,
) -> impl IntoView {
    let code = Signal::stored(code);

    view! {
        <div class="bg-codeblock-light dark:bg-codeblock-dark rounded-lg p-4 relative shadow-sm">
            <CopyButton
                class="absolute top-2 right-2 !mr-0"
                to_copy=code
            />
            <pre class="overflow-x-auto cursor-text text-wrap">{code}</pre>
        </div>
    }
}
