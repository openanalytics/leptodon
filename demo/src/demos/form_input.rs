use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_components::button::Button;
use leptos_components::button::ButtonType;
use leptos_components::form_input::FormInput;
use leptos_components::heading::Heading4;
use leptos_components::input::InputType;
use leptos_components::input::PasswordInput;
use leptos_components::input::TextInput;
use leptos_components::layout::FixedCenterColumn;
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

            <leptos_components::form_input::FormInputDocs />
            <leptos_components::input::TextInputDocs />
            <leptos_components::input::PasswordInputDocs />
        </FixedCenterColumn>
    }
}
