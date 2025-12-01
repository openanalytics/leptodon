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
use cfg_if::cfg_if;
use derive_more::Display;
use http::status::StatusCode;
use leptos::{error::Errors, html::ElementChild, prelude::{For, Get, RwSignal}, *};
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Display)]
pub enum ConfigurationError {
    #[display("Error while constructing settings from file and environment: {}", _0)]
    SettingsInitializationError(String),
}

// A basic function to display errors served by the error boundaries. Feel free to do more complicated things
// here than just displaying them
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };

    // Get Errors from Signal
    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .get()
        .into_iter()
        .filter_map(|(_, error)| error.downcast_ref::<AppError>().cloned())
        .collect();

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    cfg_if! {
      if #[cfg(feature="ssr")]{
        use leptos::context::use_context;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response{
          response.set_status(errors[0].status_code());
        }
      }
    }

    view! {
        <h1>"Errors"</h1>
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each=move || { errors.clone().into_iter().enumerate() }
            // a unique key for each item as a reference
            key=|(index, _error)| *index
            // renders each item to a view
            children=move |error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}
