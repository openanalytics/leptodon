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
use axum::{
    body::Body,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use leptos::error::Errors;
use leptos::view;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::errors::{AppError, ErrorTemplate};

pub async fn file_and_error_handler(uri: Uri, request: Request<Body>) -> AxumResponse {
    let response = get_asset(uri.clone(), "/").await;

    if let Ok(ok_response) = response
        && ok_response.status() == StatusCode::OK
    {
        return ok_response.into_response();
    }
    let mut errors = Errors::default();
    errors.insert_with_default_key(AppError::NotFound);
    let handler = leptos_axum::render_app_to_stream(
        move || view! { <ErrorTemplate outside_errors=errors.clone() /> },
    );
    handler(request).await.into_response()
}

async fn get_asset(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    match uri.path() {
        "/style.css" => get_static_file(uri.clone(), "./style").await,
        _ => get_static_file(uri.clone(), root).await,
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    log::info!("serving from dir: {root}");
    ServeDir::new(root)
        .oneshot(req)
        .await
        .map(|res| res.into_response())
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {err}"),
            )
        })
}
