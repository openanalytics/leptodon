#![recursion_limit = "256"]

pub mod app;
pub mod demo_table;
pub mod errors;
#[cfg(feature = "ssr")]
pub mod fallback;
pub mod forms;
pub mod gen_icons;
pub mod group_table;
pub mod ical_property;
mod testcases;
pub mod web_calendar;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
