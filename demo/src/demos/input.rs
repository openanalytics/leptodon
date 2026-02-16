use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::IntoAny;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_components::button::AddButton;
use leptos_components::button::DeleteButton;
use leptos_components::button::DownloadButton;
use leptos_components::button::EditButton;
use leptos_components::heading::Heading4;
use leptos_components::input::GenericInput;
use leptos_components::input::InputType;
use leptos_components::input::PasswordInput;
use leptos_components::input::TextInput;
use leptos_components::layout::FixedCenterColumn;
use leptos_meta::Title;

#[generate_codeblock(TextInputExample)]
#[component]
pub fn TextInputDemo() -> impl IntoView {
    let value = RwSignal::new(String::default());

    view! {
        <b>
            {move ||
                format!("Hello {}!", value.get().to_string())
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
                    view!{ <p>Result = {sum.a + sum.b}</p> }.into_any()
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
            <p>
                Include personal information in the hazards vec to avoid easy to guess passwords.
            </p>
            <PasswordInputExample />

            <Heading4 anchor="generic-input">Generic Input</Heading4>
            <p>
                "Internally all inputs try to use GenericInput<T, E> where T is the value of the type you are interested in and E a displayable error type.
                On the web users always enter text so you need to provider a parser and formatter to convert between String -> T and T -> String.
                In case parsing fails you may produce an Result::Err(error_of_type_E) which the GenericInput will display underneith itself.
                When you want to place elements NEXT to the GenericInput you will have difficulties with the Label and Feedback, for this situation see <FormInput>."
            </p>
            <GenericInputExample/>

            <leptos_components::input::TextInputDocs />
            <leptos_components::input::PasswordInputDocs />
            <leptos_components::input::GenericInputDocs />
        </FixedCenterColumn>
    }
}
