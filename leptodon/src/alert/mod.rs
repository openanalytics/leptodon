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
use crate::button::{Button, ButtonAppearance};
use crate::icon;
use leptodon_proc_macros::generate_docs;
use leptos::prelude::{
    Children, ClassAttribute, ElementChild, Get, GlobalAttributes, IntoAny, MaybeProp, RwSignal,
    Set, Show, ShowLet, Signal,
};
use leptos::{IntoView, component, view};
use web_sys::MouseEvent;

use crate::class_list;
use crate::icon::Icon;
use crate::icon::icon_data::IconRef;
use crate::util::callback::ArcOneCallback;
use crate::util::styling::GenericTheme;

const ALERT_CLASS: &str = "relative text-sm rounded-lg";
const ALERT_PADDING_CLASS: &str = "p-4 my-2";

#[generate_docs]
#[component]
pub fn Alert(
    /// Extra styling
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Preset styles for the alert.
    #[prop(optional)]
    theme: AlertTheme,
    /// Text shown before *children*, slightly bold.
    #[prop(optional, into)]
    prefix: MaybeProp<String>,
    /// Icon shown inside the alert, before the children
    #[prop(optional, into)]
    icon: MaybeProp<IconRef>,
    /// Whether the alert should have a border
    #[prop(optional)]
    border: bool,
    /// Whether the alert should have a button to dismiss it.
    #[prop(optional)]
    dismissable: bool,
    /// Ran on dismissal request
    #[prop(optional, into)]
    on_dismiss: MaybeProp<ArcOneCallback<MouseEvent>>,
    /// Whether to have a default amount of spacing above and below the alert.
    #[prop(default = true)]
    default_spacing: bool,
    /// Usually text content.
    children: Children,
) -> impl IntoView {
    let dismissed = RwSignal::new(false);

    view! {
        <div
            class=class_list!(
                ALERT_CLASS,
                class,
                theme.base_class(),
                (theme.border_class(), border),
                ("gap-1", dismissable),
                ("hidden", move || dismissed.get()),
                (ALERT_PADDING_CLASS, move || default_spacing)
            )
             role="alert"
        >
            <ShowLet
                some=move || icon.get()
                fallback=|| ()
                let:icon
            >
                <Icon icon class=class_list!(
                    "w-4 h-4 inline me-2 translate-y-[-0.05rem]",
                    theme.stroke_class()
                ) />
            </ShowLet>
            <ShowLet
                some=move || prefix.get()
                fallback=|| ()
                let:prefix
            >
                <span class="font-medium">{prefix}" - "</span>
            </ShowLet>
            {children()}
            <Show when=move || dismissable fallback=|| ().into_any()>
                <Button
                    class="absolute right-4 top-4"
                    icon=icon::CloseIcon()
                    appearance=ButtonAppearance::Minimal
                    on_click=move |e| {
                        dismissed.set(true);
                        if let Some (on_dismiss) = on_dismiss.get() { on_dismiss(e) }
                    }
                />
            </Show>
        </div>
    }
}

#[generate_docs]
#[component]
pub fn InfoAlert(
    /// Extra styling
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Text shown before *children*, slightly bold.
    #[prop(default = "Info".to_string().into(), into)]
    prefix: Signal<String>,
    /// Icon shown inside the alert, before the children
    #[prop(default = icon::InfoIcon())]
    icon: IconRef,
    /// Whether the alert should have a border
    #[prop(optional)]
    border: bool,
    /// Whether the alert should have a button to dismiss it.
    #[prop(optional)]
    dismissable: bool,
    /// Ran on dismissal request
    #[prop(optional, into)]
    on_dismiss: MaybeProp<ArcOneCallback<MouseEvent>>,
    /// Whether to have a default amount of spacing above and below the alert.
    #[prop(default = true)]
    default_spacing: bool,
    /// Usually text content.
    children: Children,
) -> impl IntoView {
    view! {
        <Alert class theme=AlertTheme::Brand prefix icon border dismissable on_dismiss default_spacing children />
    }
}

#[generate_docs]
#[component]
pub fn WarnAlert(
    /// Extra styling
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Text shown before *children*, slightly bold.
    #[prop(default = "Warn".to_string().into(), into)]
    prefix: Signal<String>,
    /// Icon shown inside the alert, before the children
    #[prop(default = icon::WarnIcon())]
    icon: IconRef,
    /// Whether the alert should have a border
    #[prop(optional)]
    border: bool,
    /// Whether the alert should have a button to dismiss it.
    #[prop(optional)]
    dismissable: bool,
    /// Ran on dismissal request
    #[prop(optional, into)]
    on_dismiss: MaybeProp<ArcOneCallback<MouseEvent>>,
    /// Whether to have a default amount of spacing above and below the alert.
    #[prop(default = true)]
    default_spacing: bool,
    /// Usually text content.
    children: Children,
) -> impl IntoView {
    view! {
        <Alert class theme=AlertTheme::Warning prefix icon border dismissable on_dismiss default_spacing children />
    }
}

#[generate_docs]
#[component]
pub fn ErrorAlert(
    /// Extra styling
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Text shown before *children*, slightly bold.
    #[prop(default = "Error".to_string().into(), into)]
    prefix: Signal<String>,
    /// Icon shown inside the alert, before the children
    #[prop(default = icon::RejectedIcon())]
    icon: IconRef,
    /// Whether the alert should have a border
    #[prop(optional)]
    border: bool,
    /// Whether the alert should have a button to dismiss it.
    #[prop(optional)]
    dismissable: bool,
    /// Ran on dismissal request
    #[prop(optional, into)]
    on_dismiss: MaybeProp<ArcOneCallback<MouseEvent>>,
    /// Whether to have a default amount of spacing above and below the alert.
    #[prop(default = true)]
    default_spacing: bool,
    /// Usually text content.
    children: Children,
) -> impl IntoView {
    view! {
        <Alert class theme=AlertTheme::Danger prefix icon border dismissable on_dismiss default_spacing children />
    }
}

#[generate_docs]
#[component]
pub fn SuccessAlert(
    /// Extra styling
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Text shown before *children*, slightly bold.
    #[prop(default = "Success".to_string().into(), into)]
    prefix: Signal<String>,
    /// Icon shown inside the alert, before the children
    #[prop(default = icon::ApprovedIcon())]
    icon: IconRef,
    /// Whether the alert should have a border
    #[prop(optional)]
    border: bool,
    /// Whether the alert should have a button to dismiss it.
    #[prop(optional)]
    dismissable: bool,
    /// Ran on dismissal request
    #[prop(optional, into)]
    on_dismiss: MaybeProp<ArcOneCallback<MouseEvent>>,
    /// Whether to have a default amount of spacing above and below the alert.
    #[prop(default = true)]
    default_spacing: bool,
    /// Usually text content.
    children: Children,
) -> impl IntoView {
    view! {
        <Alert class theme=AlertTheme::Success prefix icon border dismissable on_dismiss default_spacing children />
    }
}

#[derive(Default, Copy, Clone)]
pub enum AlertTheme {
    #[default]
    Brand,
    Secondary,
    Transparent,
    Danger,
    Success,
    Warning,
}

impl AlertTheme {
    fn generic_theme(&self) -> GenericTheme {
        match self {
            AlertTheme::Brand => GenericTheme::Brand,
            AlertTheme::Secondary => GenericTheme::Secondary,
            AlertTheme::Transparent => GenericTheme::Transparent,
            AlertTheme::Danger => GenericTheme::Danger,
            AlertTheme::Success => GenericTheme::Success,
            AlertTheme::Warning => GenericTheme::Warning,
        }
    }

    // Color theme
    pub fn base_class(&self) -> &'static str {
        self.generic_theme().base_class()
    }

    pub fn stroke_class(&self) -> &'static str {
        self.generic_theme().stroke_class()
    }

    // Border and their colors
    pub fn border_class(&self) -> &'static str {
        self.generic_theme().border_class()
    }
}
