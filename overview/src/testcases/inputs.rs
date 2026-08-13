use leptodon::button::Button;
use leptodon::button::ButtonType;
use leptodon::card::Card;
use leptodon::card::CardField;
use leptodon::card::CardSection;
use leptos::prelude::CustomAttribute;
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
use leptodon::heading::Heading4;
use leptodon::input::NumberInput;
use leptodon::input::NumberInputConfigProps;
use leptodon::input::OptNumberInput;
use leptodon::input::OptionalU32;
use leptodon::input::TextInput;
use leptodon::input::TextInputConfigProps;
use leptos::prelude::BindAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;

#[component]
pub fn TestInputs() -> impl IntoView {
    let text_value = RwSignal::new(String::new());
    let u32value = RwSignal::new(0u32);
    let optu32value = RwSignal::new(OptionalU32::default());
    let i128value = RwSignal::new(-1i128);
    let f64value = RwSignal::new(std::f64::consts::PI);
    let value_bind = RwSignal::new("".to_string());
    view! {
        <Title text="Test Inputs"/>
        <Card>
            <CardSection>
                <Heading4>"TextInput"</Heading4>
                <form>
                    <CardField name="value">
                        <p id="text-input-display">
                            {move || text_value.get()}
                        </p>
                    </CardField>

                    <TextInput
                        id="text-input"
                        class="my-3"
                        value=text_value
                        text_config=TextInputConfigProps::builder()
                            .max_len(10)
                            .trim(true)
                            .build()
                    />
                    <Button button_type=ButtonType::Submit>"Submit"</Button>
                </form>
            </CardSection>
        </Card>

        <Card>
            <CardSection>
                <Heading4>"NumberInput<u32>"</Heading4>
                <form>
                    <CardField name="value">
                        <p id="u32-input-display">
                            {move || u32value.get()}
                        </p>
                    </CardField>
                    <NumberInput<u32>
                        required=true
                        label="Integer between 0 and 10"
                        id="u32-input"
                        class="my-3"
                        value=u32value
                        number_config={
                            NumberInputConfigProps::<u32>::builder()
                                .max(10)
                                .trim(true)
                                .build()
                        }
                    />
                    <Button button_type=ButtonType::Submit>"Submit"</Button>
                </form>
            </CardSection>
        </Card>

        <Card>
            <CardSection>
                <Heading4>"NumberInput<i128>"</Heading4>
                <form>
                    <CardField name="value">
                        <p id="i128-input-display">
                            {move || i128value.get()}
                        </p>
                    </CardField>
                    <NumberInput<i128>
                        label = "Integer between -100 and 10"
                        id="i128-input"
                        class="my-3"
                        value=i128value
                        number_config={
                            NumberInputConfigProps::<i128>::builder()
                                .max(10)
                                .min(-100)
                                .trim(true)
                                .build()
                        }
                    />
                    <Button button_type=ButtonType::Submit>"Submit"</Button>
                </form>
            </CardSection>
        </Card>
        <Card>
            <CardSection>
                <Heading4>"NumberInput<f64>"</Heading4>
                <form>
                    <CardField name="value">
                        <p id="f64-input-display">
                            {move || f64value.get()}
                        </p>
                    </CardField>
                    <NumberInput<f64>
                        label = "Decimal between -2.01 and 10.05"
                        id="f64-input"
                        class="my-3"
                        value=f64value
                        number_config={
                            NumberInputConfigProps::<f64>::builder()
                                .max(10.05)
                                .min(-2.01)
                                .step(0.01)
                                .trim(true)
                                .build()
                        }
                    />
                    <Button button_type=ButtonType::Submit>"Submit"</Button>
                </form>
            </CardSection>
        </Card>

        <Card>
            <CardSection>
                <Heading4>"OptNumberInput<OptionalU32, u32>"</Heading4>
                <form data-testid="opt-u32-form">
                    <CardField name="value">
                        <p id="opt-u32-input-display">
                            {move || optu32value.get().to_string()}
                        </p>
                    </CardField>

                    <OptNumberInput<OptionalU32, u32>
                        required=true
                        label="Integer between 0 and 10"
                        id="opt-u32-input"
                        class="my-3"
                        value=optu32value
                        number_config={
                            NumberInputConfigProps::<u32>::builder()
                                .max(10)
                                .trim(true)
                                .build()
                        }
                    />
                    <Button button_type=ButtonType::Submit>"Submit"</Button>
                </form>
            </CardSection>
        </Card>

        <Heading4>"Debugging numbers input"</Heading4>
        <input type="number" inputmode="numeric" min="-10" max="10" step="0.1" />
        <input type="number" inputmode="numeric" bind:value=value_bind min="-10" max="10" step="0.1" />
    }
}
