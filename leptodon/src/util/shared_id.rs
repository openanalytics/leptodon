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
use std::sync::atomic::{AtomicU32, Ordering};

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use leptos::server::{SharedValue, codee::string::FromToStringCodec};

static GLOBAL_ID_GEN: AtomicU32 = AtomicU32::new(0);

/// Increments and gets the global id atomic
pub fn shared_id() -> SharedValue<String, FromToStringCodec> {
    // Wraps around on overflow.
    let id_nb = GLOBAL_ID_GEN.fetch_add(1, Ordering::Relaxed);

    SharedValue::new_str(|| BASE64_URL_SAFE_NO_PAD.encode(id_nb.to_le_bytes()))
}
