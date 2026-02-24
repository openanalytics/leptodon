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
