use leptos::prelude::ClassAttribute;
use leptos::prelude::CollectView;
use leptos::prelude::Effect;
use leptos::prelude::Get;
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
use attr_docgen::generate_docs;
use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::MaybeProp;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::Notify;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Track;
use leptos::prelude::Trigger;
use leptos::{IntoView, component, view};
use web_sys::HtmlInputElement;

use crate::class_list;
use crate::form_input::Label;
const FILE_UPLOAD_CLASS: &str = "cursor-pointer bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-heading text-sm rounded-lg block w-full !shadow-sm placeholder:text-gray-600 dark:placeholder:text-gray-400
dark:focus:ring-gray-800 outline-offset-[-1px] outline-[5px]
!outline-none focus:z-10 focus:outline-oa-blue ";

/// Dropzone for file upload, can also be clicked to open a file-picker dialog.
#[generate_docs]
#[component]
pub fn FileUpload(
    #[prop(optional, into)] id: MaybeProp<String>,
    /// Used to identify the files in a form submission.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional)] required: bool,
    /// Allow selecting multiple files to upload in this input?
    #[prop(optional)]
    multiple: bool,
    /// The allowed types to upload (e.g. "image/*,.pdf" to allow any image type and files with the pdf extension)
    /// Omitting this option allows every file type, remember to sanitize uploaded user-input.
    #[prop(optional, into)]
    accept: MaybeProp<String>,
) -> impl IntoView {
    let input_ref = NodeRef::new();
    let trigger_list_refresh = Trigger::new();
    let has_multiple_entries = RwSignal::new(false);
    Effect::new(move || {
        trigger_list_refresh.track();
        let opt_input: Option<HtmlInputElement> = input_ref.get();
        if let Some(input) = opt_input
            && let Some(files) = input.files()
        {
            has_multiple_entries.set(files.length() > 1);
        } else {
            has_multiple_entries.set(false);
        }
    });
    view! {
        <Label label=label required=required>
            <input
                class=class_list!(FILE_UPLOAD_CLASS, ("", move || has_multiple_entries.get()))
                id=id.get()
                name=name.get()
                type="file"
                accept=accept.get()
                node_ref=input_ref
                multiple=multiple
                on:change=move |_| trigger_list_refresh.notify()
             />
            {move || {
                if let Some(input) = input_ref.get() && let Some(files) = input.files() && has_multiple_entries.get() {
                    let list_items = (0..files.length()).filter_map(|i| files.get(i)).map(|file| {
                        view!{
                            // Browsers render the list markers differently so that's why this uses before:
                            <li class="before:content-['•'] before:relative before:left-[-0.35rem] before:text-gray-300">{file.name()}</li>
                        }.into_any()
                    }).collect_view().into_any();
                    view! {
                        <ul role="list" class="z-[-1] pt-2 space-y-2 text-gray-900 dark:text-gray-50 border-s-2 ms-3">
                            {list_items}
                        </ul>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </Label>
    }
}
