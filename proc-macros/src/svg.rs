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
//! Compile-time SVG templating.
//!
//! [`svg!`] parses its argument as RSTML markup and emits a `&'static str`
//! literal containing the rendered SVG. Because the markup is validated by the
//! parser at compile time, malformed SVG is a hard compile error, and because
//! the result is baked in as a string literal nothing is (re)built at runtime.
use std::fmt::Write;

use proc_macro::TokenStream;
use quote::quote;
use rstml::{
    Parser, ParserConfig,
    node::{Infallible, KeyedAttributeValue, Node, NodeAttribute, NodeElement},
};
use syn::spanned::Spanned;

/// Renders an RSTML/SVG [`Node`] tree into a plain HTML/SVG string.
///
/// Only static markup is supported: any dynamic `{...}` block or non-literal
/// attribute value records an error, since the result must be a `&'static str`.
struct Renderer {
    output: String,
    /// The first fatal problem encountered while rendering, if any.
    error: Option<String>,
}

impl Renderer {
    fn new() -> Self {
        Self {
            output: String::new(),
            error: None,
        }
    }

    fn fail(&mut self, message: impl Into<String>) {
        // Keep the first diagnostic so it is the one surfaced to the user.
        if self.error.is_none() {
            self.error = Some(message.into());
        }
    }

    fn render_nodes(&mut self, nodes: &[Node<Infallible>]) {
        for node in nodes {
            self.render_node(node);
        }
    }

    fn render_node(&mut self, node: &Node<Infallible>) {
        if self.error.is_some() {
            return;
        }
        match node {
            Node::Element(element) => self.render_element(element),
            Node::Fragment(fragment) => self.render_nodes(&fragment.children),
            Node::Text(text) => {
                let _ = write!(self.output, "{}", text.value_string());
            }
            Node::RawText(raw) => {
                let _ = write!(self.output, "{}", raw.to_string_best());
            }
            Node::Comment(comment) => {
                let _ = write!(self.output, "<!-- {} -->", comment.value.to_string_best());
            }
            Node::Doctype(doctype) => {
                let _ = write!(self.output, "<!DOCTYPE {}>", doctype.value.to_string_best());
            }
            Node::Block(_) => self.fail(
                "`svg!` only supports static markup: dynamic `{...}` expressions cannot be baked \
                 into a compile-time string",
            ),
            Node::Custom(_) => self.fail("unsupported custom node in `svg!` markup"),
        }
    }

    fn render_element(&mut self, element: &NodeElement<Infallible>) {
        if self.error.is_some() {
            return;
        }
        let name = element.name().to_string();
        let _ = write!(self.output, "<{name}");
        for attribute in element.attributes() {
            self.render_attribute(attribute);
        }
        if element.children.is_empty() {
            // No children: emit a self-closing tag, which is well-formed XML/SVG.
            let _ = self.output.write_str("/>");
        } else {
            let _ = self.output.write_char('>');
            self.render_nodes(&element.children);
            let _ = write!(self.output, "</{name}>");
        }
    }

    fn render_attribute(&mut self, attribute: &NodeAttribute) {
        if self.error.is_some() {
            return;
        }
        match attribute {
            NodeAttribute::Attribute(keyed) => {
                let key = keyed.key.to_string();
                let _ = write!(self.output, " {key}");
                match &keyed.possible_value {
                    // A bare attribute flag (e.g. `<path fill-rule="evenodd">`-style booleans).
                    KeyedAttributeValue::None => {}
                    KeyedAttributeValue::Binding(_) => self.fail(
                        "`svg!` does not support closure/binding attribute values in static markup",
                    ),
                    KeyedAttributeValue::Value(value) => match value.value_literal_string() {
                        Some(literal) => {
                            let _ = write!(self.output, "=\"{literal}\"");
                        }
                        None => self.fail(
                            "`svg!` attribute values must be static literals (strings or numbers); \
                             dynamic expressions cannot be baked into a compile-time string",
                        ),
                    },
                }
            }
            NodeAttribute::Block(_) => self
                .fail("`svg!` does not support dynamic `{...}` attribute spreads in static markup"),
        }
    }
}

/// Implementation of the [`crate::svg`] proc macro.
///
/// See the public [`crate::svg`] docs for usage and examples.
pub fn _svg(tokens: TokenStream) -> TokenStream {
    let tokens = proc_macro2::TokenStream::from(tokens);
    let span = tokens.span();

    // Strict parse: malformed SVG must surface as a compile error.
    let parser = Parser::new(ParserConfig::new());
    let nodes = match parser.parse_simple(tokens) {
        Ok(nodes) => nodes,
        Err(error) => {
            return syn::Error::new(error.span(), format!("`svg!`: malformed SVG: {error}"))
                .to_compile_error()
                .into();
        }
    };

    let mut renderer = Renderer::new();
    renderer.render_nodes(&nodes);

    match renderer.error {
        Some(message) => syn::Error::new(span, format!("`svg!`: {message}"))
            .to_compile_error()
            .into(),
        None => {
            // `LitStr` escapes its content and expands to a string literal,
            // i.e. a `&'static str`.
            let literal = syn::LitStr::new(&renderer.output, span);
            quote!(#literal).into()
        }
    }
}
