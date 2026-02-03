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
