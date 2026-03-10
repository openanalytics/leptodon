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
use leptodon::button::AddButton;
use leptodon::button::DeleteButton;
use leptodon::button::DownloadButton;
use leptodon::button::EditButton;
use leptodon::heading::Heading4;
use leptodon::input::FileUpload;
use leptodon::input::GenericInput;
use leptodon::input::InputType;
use leptodon::input::NumberInput;
use leptodon::input::NumberInputConfigProps;
use leptodon::input::PasswordInput;
use leptodon::input::TextInput;
use leptodon::layout::FixedCenterColumn;
use leptodon::paragraph::Paragraph;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::IntoAny;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;

#[generate_codeblock(TextInputExample)]
#[component]
pub fn TextInputDemo() -> impl IntoView {
    let value = RwSignal::new(String::default());

    view! {
        <b>
            {move ||
                format!("Hello {}!", value.get())
            }
        </b>

        <TextInput
            label="Username"
            name="username"
            placeholder="Username"
            input_type=InputType::Text value
        />
    }
}

#[generate_codeblock(PasswordInputExample)]
#[component]
pub fn PasswordInputDemo() -> impl IntoView {
    view! {
        <PasswordInput
            label="Password"
            name="password"
            placeholder="Please enter your password."
            show_eye=true
            hazards=vec!["Merlijn".to_string(), "Verstraete".to_string()]
        />
    }
}

#[generate_codeblock(NumberInputExample)]
#[component]
pub fn NumberInputDemo() -> impl IntoView {
    let value = RwSignal::new(5.5);

    view! {
        <Paragraph>
            {move || value.get()}
        </Paragraph>
        <NumberInput<f64> // <- Supports u8-128,i8-128 and f32,f64
            label = "Decimal between -2.00 and 10.15"
            class="my-3"
            value=value
            number_config={
                NumberInputConfigProps::<f64>::builder()
                    .max(10.15)
                    .min(-2.00)
                    .step(0.01)
                    .trim(true)
                    .build()
            }
        />
    }
}

#[generate_codeblock(FileUploadExample)]
#[component]
pub fn FileUploadDemo() -> impl IntoView {
    view! {
        <FileUpload
            required=true
            multiple=true
            label="Upload your receipt(s)"
            name="receipts"
            accept=".pdf"
        />
    }
}

#[generate_codeblock(GenericInputExample)]
#[component]
pub fn GenericInputDemo() -> impl IntoView {
    #[derive(Debug, Clone, Default, PartialEq)]
    struct Sum {
        a: i128,
        b: i128,
    }
    type OptSum = Option<Sum>;
    let value = RwSignal::new(None);

    // Input parser
    let parser = |to_parse: String| {
        if to_parse.is_empty() {
            return Ok(None);
        }
        if let Some((left, right)) = to_parse.split_once("+") {
            let left_parsed = match left.trim().parse::<i128>() {
                Ok(left) => left,
                Err(parse_err) => return Err(format!("Left: {parse_err:?}")),
            };

            let right_parsed = match right.trim().parse::<i128>() {
                Ok(right) => right,
                Err(parse_err) => return Err(format!("Right: {parse_err:?}")),
            };

            Ok(Some(Sum {
                a: left_parsed,
                b: right_parsed,
            }))
        } else {
            Err("Missing a + symbol. Desired format: a+b".to_string())
        }
    };

    // Input formatter
    let format = |sum: OptSum| {
        if let Some(sum) = sum {
            format!("{} + {}", sum.a, sum.b)
        } else {
            String::default()
        }
    };

    view! {
        <GenericInput<OptSum, String>
            label="Add two numbers"
            name="addition"
            placeholder="2 + 2"
            parser
            format
            value
        />
        {
            move || {
                if let Some(sum) = value.get() {
                    view!{ <Paragraph>Result = {sum.a + sum.b}</Paragraph> }.into_any()
                } else { ().into_any() }
            }
        }
    }
}

#[generate_codeblock(StyledButtonExample)]
#[component]
pub fn PremadeButtonDemo() -> impl IntoView {
    view! {
        <AddButton/>
        <EditButton/>
        <DeleteButton/>
        <DownloadButton/>
    }
}

#[component]
pub fn InputsDemoPage() -> impl IntoView {
    view! {
        <Title text="Button Components"/>

        <FixedCenterColumn>
            <Heading4 anchor="text-input">Text Input</Heading4>
            <TextInputExample />

            <Heading4 anchor="password-input">Password Input</Heading4>
            <Paragraph>
                Include personal information in the hazards vec to avoid easy to guess passwords.
            </Paragraph>
            <PasswordInputExample />

            <Heading4 anchor="number-input">Number Input</Heading4>
            <NumberInputExample/>

            <Heading4 anchor="file-upload">File Upload</Heading4>
            <FileUploadExample/>

            <Heading4 anchor="generic-input">Generic Input</Heading4>
            <Paragraph>
                "Internally all inputs try to use GenericInput<T, E> where T is the value of the type you are interested in and E a displayable error type.
                On the web users always enter text so you need to provider a parser and formatter to convert between String -> T and T -> String.
                In case parsing fails you may produce an Result::Err(error_of_type_E) which the GenericInput will display underneith itself.
                When you want to place elements NEXT to the GenericInput you will have difficulties with the Label and Feedback, for this situation see <FormInput>."
            </Paragraph>
            <GenericInputExample/>

            <leptodon::input::TextInputDocs />
            <leptodon::input::PasswordInputDocs />
            <leptodon::input::NumberInputDocs />
            <leptodon::input::FileUploadDocs />
            <leptodon::input::GenericInputDocs />
        </FixedCenterColumn>
    }
}
