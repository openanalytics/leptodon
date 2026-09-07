// Leptodon
//
// Copyright (c) 2023 lizidev
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
use leptos::html::ElementType;
use send_wrapper::SendWrapper;

/// Allows using node_refs to generic elements, these elements need to be Send to be rendered by leptos.
#[derive(Debug, Clone)]
pub struct Element {
    el: Option<SendWrapper<web_sys::Element>>,
}

impl ElementType for Element {
    type Output = web_sys::Element;

    const TAG: &'static str = "";

    const SELF_CLOSING: bool = false;

    const ESCAPE_CHILDREN: bool = false;

    const NAMESPACE: Option<&'static str> = None;

    fn tag(&self) -> &str {
        ""
    }
}

impl Element {
    pub fn get_inner(&self) -> Option<&web_sys::Element> {
        self.el.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct HtmlElement {
    el: Option<SendWrapper<web_sys::HtmlElement>>,
}

impl ElementType for HtmlElement {
    type Output = web_sys::HtmlElement;

    const TAG: &'static str = "";

    const SELF_CLOSING: bool = false;

    const ESCAPE_CHILDREN: bool = false;

    const NAMESPACE: Option<&'static str> = None;

    fn tag(&self) -> &str {
        ""
    }
}
