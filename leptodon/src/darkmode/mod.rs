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
use leptodon_proc_macros::generate_docs;
use leptos::logging::debug_log;
use leptos::oco::Oco;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::Effect;
use leptos::prelude::Get;
use leptos::prelude::Memo;
use leptos::prelude::RwSignal;
use leptos::prelude::Signal;
use leptos::server::ServerAction;
use leptos::{prelude::ServerFnError, *};
use leptos_meta::Html;
use leptos_meta::Meta;
use leptos_use::use_preferred_dark;
use std::fmt::Display;
use std::str::FromStr;

use crate::radio::RadioOption;
use crate::select::Select;

#[derive(Debug, Hash, Clone, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    Light,
    #[default]
    Dark,
    FollowSystem,
}

impl RadioOption for Theme {
    fn value(&self) -> prelude::Oco<'static, str> {
        match self {
            Theme::Light => Oco::Borrowed("light"),
            Theme::Dark => Oco::Borrowed("dark"),
            Theme::FollowSystem => Oco::Borrowed("follow_system"),
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => f.write_str("Light theme"),
            Theme::Dark => f.write_str("Dark theme"),
            Theme::FollowSystem => f.write_str("Follow system-theme"),
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
        Ok(match value {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            "follow_system" => Theme::FollowSystem,
            _ => return Err(()),
        })
    }
    type Err = ();
}

const THEME_COOKIE: &str = "theme";

#[server(UpdateTheme, "/api")]
pub async fn update_theme(new_theme: Theme) -> Result<Theme, ServerFnError> {
    use axum::http::{HeaderMap, HeaderValue, header::SET_COOKIE};
    use leptos::context::use_context;
    use leptos_axum::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>().expect("to have leptos_axum::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    let theme_value = new_theme.value();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "{THEME_COOKIE}={theme_value}; Path=/; Max-Age=186400; SameSite=Strict"
        ))
        .expect("to create header value"),
    );
    response_parts.headers = headers;
    response.overwrite(response_parts);
    Ok(new_theme)
}

pub fn fetch_ssr_tailwind_class() -> String {
    let theme = initial_theme_from_cookie();
    if theme == Theme::FollowSystem && !browser_prefers_darkmode().get() {
        return "".to_string();
    }
    debug_log!("Final theme: {theme:?}");
    // console_log(format!("Final theme: {theme:?}").as_str());
    let resulting_theme = match theme {
        Theme::Light => "light",
        Theme::FollowSystem if browser_prefers_darkmode().get() => "dark",
        Theme::FollowSystem => "light",
        Theme::Dark => "dark",
    };
    debug_log!("Resulting theme: {resulting_theme:?}");
    // console_log(format!("Resulting theme: {resulting_theme:?}").as_str());
    resulting_theme.to_string()
}

#[generate_docs]
#[component]
pub fn ThemeSelector() -> impl IntoView {
    let update_theme_action: ServerAction<UpdateTheme> = ServerAction::new();
    let cookie_theme = initial_theme_from_cookie();
    let browser_prefers_dark = browser_prefers_darkmode();
    // Bound to the html select.
    let selected_theme = RwSignal::new(cookie_theme);
    let resulting_light_dark = Memo::new(move |_| {
        let theme = selected_theme.get();
        // console_log(format!("Resulting DL theme: {resulting_theme:?}").as_str());
        match theme {
            Theme::Light => "light",
            Theme::FollowSystem if browser_prefers_dark.get() => "dark light",
            Theme::FollowSystem => "light dark",
            Theme::Dark => "dark",
        }
    });

    let resulting_dark = Memo::new(move |_| {
        let theme = selected_theme.get();
        debug_log!("Final theme: {theme:?}");
        // console_log(format!("Final theme: {theme:?}").as_str());
        let resulting_theme = match theme {
            Theme::Light => "light",
            Theme::FollowSystem if browser_prefers_dark.get() => "dark",
            Theme::FollowSystem => "light",
            Theme::Dark => "dark",
        };
        debug_log!("Resulting theme: {resulting_theme:?}");
        // console_log(format!("Resulting theme: {resulting_theme:?}").as_str());
        resulting_theme
    });

    Effect::watch(
        move || selected_theme.get(),
        move |theme, prev_theme, _| {
            if Some(theme) == prev_theme {
                return;
            }
            debug_log!("Updating theme from {prev_theme:?} to {theme}");
            let selected_theme = theme.clone();
            update_theme_action.dispatch(UpdateTheme {
                new_theme: selected_theme,
            });
        },
        false,
    );

    view! {
        <Html {..} class=move || {
            debug_log!("{:?}", resulting_dark.get());
            if resulting_dark.get() == "" {
                fetch_ssr_tailwind_class().to_string()
            } else {
                resulting_dark.get().to_string()
            }
        } />
        <Meta
            name="color-scheme"
            content=move || resulting_light_dark.get()
        />
        <Select<Theme>
            required=true
            name="theme"
            selected=selected_theme
            options=RwSignal::new(vec![Theme::Light, Theme::Dark, Theme::FollowSystem])
        />
    }
}

/// Checks whether the user's system prefers dark mode based on media queries.
/// returns None iff the browser is unavailable.
pub fn browser_prefers_darkmode() -> Signal<bool> {
    use_preferred_dark()
}

#[cfg(not(feature = "ssr"))]
pub fn initial_theme_from_cookie() -> Theme {
    use leptos::prelude::document;
    use wasm_cookies::cookies;
    use web_sys::wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    if let Some(theme) = cookies::get_raw(cookie.as_str(), "theme") {
        Theme::from(theme.as_str())
    } else {
        Theme::default()
    }
}

#[cfg(feature = "ssr")]
pub fn initial_theme_from_cookie() -> Theme {
    use axum_extra::extract::cookie::Cookie;
    use leptos::prelude::use_context;
    use std::borrow::Cow;

    let Some(headers) = use_context::<http::request::Parts>().map(|parts| parts.headers) else {
        return Theme::default();
    };

    let Some(Ok(head_value_bytes)) = headers
        .get(axum::http::header::COOKIE)
        .map(|value| value.to_str())
    else {
        debug_log!("Failed to find cookie header");
        return Theme::default();
    };
    let parseable_value = Cow::from(head_value_bytes.to_string());
    let found = Cookie::split_parse_encoded(parseable_value).find_map(|a| match a {
        Ok(cookie) => {
            if cookie.name() != THEME_COOKIE {
                None
            } else if let Ok(theme) = Theme::from_str(cookie.value_trimmed()) {
                Some(theme)
            } else {
                debug_log!("Failed to decode {}={}", cookie.name(), cookie.value());
                None
            }
        }
        _ => None,
    });
    found.unwrap_or_default()
}
