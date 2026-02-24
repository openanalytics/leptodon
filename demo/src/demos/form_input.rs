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
use attr_docgen::generate_codeblock;
use leptodon::button::Button;
use leptodon::button::ButtonType;
use leptodon::form_input::FormInput;
use leptodon::heading::Heading4;
use leptodon::input::InputType;
use leptodon::input::PasswordInput;
use leptodon::input::TextInput;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;
use leptos_router::components::Form;

#[generate_codeblock(LoginFormInputExample)]
#[component]
pub fn LoginFormInputDemo() -> impl IntoView {
    view! {
        <Form action="./">
            <div class="p-4">
                <FormInput<String> label="Email address" required=true>
                    <TextInput name="email" placeholder="localpart@domain" input_type=InputType::Email />
                </FormInput<String>>
                // <String> is the feedback error type of the GenericInput inside PasswordInput.
                <FormInput<String> label="Password" required=true>
                    <PasswordInput name="password" placeholder="*******************" hazards=vec!["YourName".to_string()] show_eye=true />
                </FormInput<String>>
                <Button button_type=ButtonType::Submit>"Submit"</Button>
            </div>
        </Form>
    }
}

#[component]
pub fn FormInputDemoPage() -> impl IntoView {
    view! {
        <Title text="FormInput"/>

        <FixedCenterColumn>
            <Heading4 anchor="login-form-input">"Login FormInput"</Heading4>
            <p>
                "Form inputs should be used for input- and button-groups that require a label or form-feedback."
                <br/>
                "Some elements with postfix labels should not be labelled via <FormInput<E>>"
            </p>
            <LoginFormInputExample />

            <leptodon::form_input::FormInputDocs />
            <leptodon::input::TextInputDocs />
            <leptodon::input::PasswordInputDocs />
        </FixedCenterColumn>
    }
}
