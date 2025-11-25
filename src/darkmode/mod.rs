// Timesheets
//
// Copyright (C) 2023-2025 Open Analytics NV
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

use std::str::FromStr;
use std::fmt::Display;

use leptos::leptos_dom::logging::console_log;
use leptos::logging::debug_log;
use leptos::logging::log;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::document;
use leptos::prelude::window;
use leptos::server::ServerAction;
use leptos::{prelude::ServerFnError, *};
use leptos_meta::Meta;

use crate::select::Select;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    FollowSystem,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => f.write_str("light"),
            Theme::Dark => f.write_str("dark"),
            Theme::FollowSystem => f.write_str("follow_system"),
        }
    }
}

impl From<&str> for Theme {
    fn from(value: &str) -> Theme {
        Theme::from_str(value).unwrap_or_default()
    }
}

impl FromStr for Theme {
    /// Err when value cannot be mapped to a theme
    fn from_str(value: &str) -> Result<Theme, ()> {
        Ok( match value {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            "followsystem" => Theme::FollowSystem,
            _ => return Err(())
        })
    }
    type Err = ();
}

#[server(UpdateTheme, "/api")]
pub async fn update_theme(new_theme: Theme) -> Result<Theme, ServerFnError> {
    use axum::http::{HeaderMap, HeaderValue, header::SET_COOKIE};
    use leptos::context::use_context;
    use leptos_axum::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>().expect("to have leptos_axum::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("theme={new_theme}; Path=/"))
            .expect("to create header value"),
    );
    response_parts.headers = headers;
    response.overwrite(response_parts);
    Ok(new_theme)
}

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let update_theme_action: ServerAction<UpdateTheme> = ServerAction::new();
    let cookie_theme = initial_theme_from_cookie();
    let selected_theme = RwSignal::new(cookie_theme.to_string());
    let resulting_light_dark = move || {
        debug_log!("cookie_theme: {}", cookie_theme);
        debug_log!("{}", selected_theme.get());

        let theme = Theme::from(selected_theme.get().as_str());
        debug_log!("{}", theme);
        let resulting_theme = match theme {
            Theme::Light => "light",
            Theme::FollowSystem if browser_prefers_darkmode() != Some(true) => "light dark",
            Theme::FollowSystem => "dark light",
            Theme::Dark => "dark",
        };
        debug_log!("Resulting_theme: {resulting_theme}");
        resulting_theme
    };

    Effect::watch(
        move || selected_theme.get(),
        move |theme, prev_theme, _| {
            if Some(theme) == prev_theme {
                return;
            }
            debug_log!("Updating theme from {prev_theme:?} to {theme}");
            let selected_theme = Theme::from(theme.as_str());
            update_theme_action.dispatch(UpdateTheme {
                new_theme: selected_theme,
            });
        },
        false,
    );

    view! {
        <Meta
            name="color-scheme"
            content=resulting_light_dark
        />
        <label for="theme">Choose theme:</label>
        <Select
            name="theme"
            value=selected_theme
        >
            <option value="light">Light</option>
            <option value="dark">Dark</option>
            <option value="follow_system">Follow System</option>
        </Select>
    }
}

/// Checks whether the user's system prefers dark mode based on media queries.
/// returns None iff the browser is unavailable.
#[cfg(not(feature = "ssr"))]
pub fn browser_prefers_darkmode() -> Option<bool> {
    let prefers_darkmode = window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .map(|media| media.matches())
        .unwrap_or_default();
    Some(prefers_darkmode)
}

#[cfg(feature = "ssr")]
pub fn browser_prefers_darkmode() -> Option<bool> {
    None
}

#[cfg(not(feature = "ssr"))]
pub fn initial_theme_from_cookie() -> Theme {
    use wasm_cookies::cookies;
    use web_sys::wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    if let Some(theme) = cookies::get_raw(cookie.as_str(), "theme") {
        Theme::from(theme.as_str())
    } else {
        Theme::FollowSystem
    }
}

#[cfg(feature = "ssr")]
pub fn initial_theme_from_cookie() -> Theme {
    use leptos::server::codee::string::FromToStringCodec;
    use leptos_use::use_cookie;
    
    let (read, _write) = use_cookie::<Theme, FromToStringCodec>("theme");
    read.get().unwrap_or_default()
}
