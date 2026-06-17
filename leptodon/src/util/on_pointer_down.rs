// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
// Copyright (c) 2023 Synphonyte
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
use leptos_use::core::{ElementsMaybeSignal, IntoElementMaybeSignal, IntoElementsMaybeSignal};

cfg_if! {
    if #[cfg(not(feature = "ssr"))] {
    // if #[cfg(feature = "csr")] {
        use leptos::leptos_dom::helpers::document;
        use leptos::leptos_dom::helpers::event_target;
        use leptos::leptos_dom::helpers::window;
        use leptos::reactive::traits::GetUntracked;
        use leptos_use::UseEventListenerOptions;
        use leptos_use::use_event_listener_with_options;
        use leptos_use::use_window;
        use std::{
            cell::Cell,
            rc::Rc,
            sync::{LazyLock, RwLock},
        };
        use web_sys::wasm_bindgen::JsCast;
    }
}

#[cfg(not(feature = "ssr"))]
// #[cfg(feature = "csr")]
static IOS_WORKAROUND: RwLock<bool> = RwLock::new(false);

#[cfg(not(feature = "ssr"))]
// #[cfg(feature = "csr")]
pub static IS_IOS: LazyLock<bool> = LazyLock::new(|| {
    if let Some(Ok(user_agent)) = use_window().navigator().map(|n| n.user_agent()) {
        user_agent.contains("iPhone") || user_agent.contains("iPad") || user_agent.contains("iPod")
    } else {
        false
    }
});

/// Options for [`on_pointer_down_outside_with_options`].
#[derive(Clone)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct OnPointerDownOutsideOptions {
    /// List of elementss that should not trigger the callback. Defaults to `[]`.
    ignore: ElementsMaybeSignal<web_sys::EventTarget>,

    /// Use capturing phase for internal event listener. Defaults to `true`.
    capture: bool,

    /// Run callback if focus moves to an iframe. Defaults to `false`.
    detect_iframes: bool,
}

impl Default for OnPointerDownOutsideOptions {
    fn default() -> Self {
        Self {
            ignore: Vec::<web_sys::EventTarget>::new().into_elements_maybe_signal(),
            capture: true,
            detect_iframes: false,
        }
    }
}

impl OnPointerDownOutsideOptions {
    /// List of elements that should not trigger the callback. Defaults to `[]`.
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    pub fn ignore<M>(self, ignore: impl IntoElementsMaybeSignal<web_sys::EventTarget, M>) -> Self {
        Self {
            ignore: ignore.into_elements_maybe_signal(),
            ..self
        }
    }
}

/// Version of [leptos_use::on_click_outside_with_options] that reacts on pointer-down instead of pointer-up, takes an [OnPointerDownOutsideOptions].
/// See [leptos_use::on_click_outside] for more details.
#[cfg_attr(feature = "ssr", allow(unused_variables))]
pub fn on_pointer_down_outside_with_options<El, M, F>(
    target: El,
    handler: F,
    options: OnPointerDownOutsideOptions,
) -> impl FnOnce() + Clone + Send + Sync
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
    F: FnMut(web_sys::Event) + Clone + 'static,
{
    #[cfg(feature = "ssr")]
    {
        || {}
    }

    #[cfg(not(feature = "ssr"))]
    // #[cfg(feature = "csr")]
    {
        use leptos_use::sendwrap_fn;

        let OnPointerDownOutsideOptions {
            ignore,
            detect_iframes,
            capture,
        } = options;

        // Fixes: https://github.com/vueuse/vueuse/issues/1520
        // How it works: https://stackoverflow.com/a/39712411
        if *IS_IOS
            && let Ok(mut ios_workaround) = IOS_WORKAROUND.write()
            && !*ios_workaround
        {
            *ios_workaround = true;
            if let Some(body) = document().body() {
                let children = body.children();
                for i in 0..children.length() {
                    use web_sys::js_sys;

                    let _ = children
                        .get_with_index(i)
                        .expect("checked index")
                        .add_event_listener_with_callback("click", &js_sys::Function::default());
                }
            }
        }

        let should_listen = Rc::new(Cell::new(true));

        let should_ignore = move |event: &web_sys::UiEvent| {
            let ignore = ignore.get_untracked();

            ignore.into_iter().flatten().any(|element| {
                event_target::<web_sys::EventTarget>(event) == *element
                    || event.composed_path().includes(element.as_ref(), 0)
            })
        };

        let target = target.into_element_maybe_signal();

        let listener = {
            let should_listen = Rc::clone(&should_listen);
            let mut handler = handler.clone();

            move |event: web_sys::UiEvent| {
                if let Some(el) = target.get_untracked() {
                    if *el == event_target(&event) || event.composed_path().includes(el.as_ref(), 0)
                    {
                        return;
                    }

                    if event.detail() == 0 {
                        should_listen.set(!should_ignore(&event));
                    }

                    if !should_listen.get() {
                        should_listen.set(true);
                        return;
                    }

                    #[cfg(debug_assertions)]
                    let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                    handler(event.into());
                }
            }
        };

        let remove_pointer_listener = {
            use leptos::ev::pointerdown;
            let mut listener = listener.clone();

            use_event_listener_with_options::<_, web_sys::Window, _, _>(
                window(),
                pointerdown,
                move |event| {
                    if let Some(el) = target.get_untracked() {
                        if !event.composed_path().includes(&el, 0) && !should_ignore(&event) {
                            listener(event.into());
                        }
                    }
                },
                UseEventListenerOptions::default()
                    .passive(true)
                    .capture(capture),
            )
        };

        let remove_blur_listener = if detect_iframes {
            use leptos::ev::blur;
            use leptos_use::use_event_listener;

            Some(use_event_listener::<_, web_sys::Window, _, _>(
                window(),
                blur,
                move |event| {
                    use std::time::Duration;

                    use leptos::leptos_dom::helpers::set_timeout_with_handle;

                    let mut handler = handler.clone();

                    let _ = set_timeout_with_handle(
                        move || {
                            if let Some(el) = target.get_untracked()
                                && let Some(active_element) = document().active_element()
                                && active_element.tag_name() == "IFRAME"
                                && !el
                                    .unchecked_ref::<web_sys::Node>()
                                    .contains(Some(&active_element.into()))
                            {
                                handler(event.into());
                            }
                        },
                        Duration::ZERO,
                    );
                },
            ))
        } else {
            None
        };

        sendwrap_fn!(once move || {
            // remove_click_listener();
            remove_pointer_listener();
            if let Some(f) = remove_blur_listener {
                f();
            }
        })
    }
}
