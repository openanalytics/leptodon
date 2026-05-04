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
use leptodon::alert::Alert;
use leptodon::alert::AlertTheme;
use leptodon::alert::ErrorAlert;
use leptodon::alert::InfoAlert;
use leptodon::alert::SuccessAlert;
use leptodon::alert::WarnAlert;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(PlainAlertsExample)]
#[component]
pub fn PlainAlertsDemo() -> impl IntoView {
    view! {
        <Alert>Plain</Alert>
    }
}

#[generate_codeblock(ThemedAlertsExample)]
#[component]
pub fn ThemedAlertsDemo() -> impl IntoView {
    view! {
        <Alert theme=AlertTheme::Brand>Brand</Alert>
        <Alert theme=AlertTheme::Danger>Danger</Alert>
        <Alert theme=AlertTheme::Secondary>Secondary</Alert>
        <Alert theme=AlertTheme::Transparent>Transparent</Alert>
        <InfoAlert>Info</InfoAlert>
        <ErrorAlert>Error</ErrorAlert>
        <WarnAlert>Warning</WarnAlert>
        <SuccessAlert>Success</SuccessAlert>
    }
}

#[generate_codeblock(DissmissableAlertsExample)]
#[component]
pub fn DismissableAlertsDemo() -> impl IntoView {
    view! {
        <Alert theme=AlertTheme::Brand dismissable=true>Brand</Alert>
        <Alert theme=AlertTheme::Danger dismissable=true>Danger</Alert>
        <Alert theme=AlertTheme::Warning dismissable=true>Warning</Alert>
        <Alert theme=AlertTheme::Secondary dismissable=true>Secondary</Alert>
        <Alert theme=AlertTheme::Success dismissable=true>Success</Alert>
        <Alert theme=AlertTheme::Transparent dismissable=true>Transparent</Alert>
        <InfoAlert dismissable=true>Info</InfoAlert>
        <ErrorAlert dismissable=true>Error</ErrorAlert>
        <WarnAlert dismissable=true>Warning</WarnAlert>
        <SuccessAlert dismissable=true>Success</SuccessAlert>
    }
}

#[generate_codeblock(BorderedAlertsExample)]
#[component]
pub fn BorderedAlertsDemo() -> impl IntoView {
    view! {
        <Alert theme=AlertTheme::Brand border=true>Brand</Alert>
        <Alert theme=AlertTheme::Danger border=true>Danger</Alert>
        <Alert theme=AlertTheme::Warning border=true>Warning</Alert>
        <Alert theme=AlertTheme::Secondary border=true>Secondary</Alert>
        <Alert theme=AlertTheme::Success border=true>Success</Alert>
        <Alert theme=AlertTheme::Transparent border=true>Transparent</Alert>
        <InfoAlert border=true>Info</InfoAlert>
        <ErrorAlert border=true>Error</ErrorAlert>
        <WarnAlert border=true>Warning</WarnAlert>
        <SuccessAlert border=true>Success</SuccessAlert>
    }
}

#[component]
pub fn AlertDemoPage() -> impl IntoView {
    view! {
        <Title text="Alert Component"/>

        <FixedCenterColumn>
            <Heading4 anchor="plain">"Plain Alert"</Heading4>
            <PlainAlertsExample />

            <Heading4 anchor="themed-alerts">"Themed Alerts"</Heading4>
            <ThemedAlertsExample />

            <Heading4 anchor="dissmissable-alerts">"Dissmissable Alerts"</Heading4>
            <DissmissableAlertsExample />

            <Heading4 anchor="bordered-alerts">"Bordered Alerts"</Heading4>
            <BorderedAlertsExample />

            <leptodon::alert::AlertDocs />
        </FixedCenterColumn>
    }
}
