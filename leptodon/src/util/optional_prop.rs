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
use leptos::prelude::*;
use std::ops::{Deref, DerefMut};

use crate::util::callback::BoxOneCallback;

pub struct OptionalProp<T>(Option<T>);

impl<T> Default for OptionalProp<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Clone> Clone for OptionalProp<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy> Copy for OptionalProp<T> {}

impl<T> OptionalProp<T> {
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        self.0.map(f)
    }

    pub fn into_option(self) -> Option<T> {
        self.0
    }
}

impl<T> Deref for OptionalProp<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OptionalProp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for OptionalProp<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl From<&str> for OptionalProp<String> {
    fn from(value: &str) -> Self {
        Self(Some(value.to_string()))
    }
}

/// TODO remove signal
impl From<&str> for OptionalProp<Signal<String>> {
    fn from(value: &str) -> Self {
        Self(Some(Signal::from(value.to_string())))
    }
}

impl From<String> for OptionalProp<Signal<String>> {
    fn from(value: String) -> Self {
        Self(Some(Signal::from(value)))
    }
}

impl<T> From<ReadSignal<T>> for OptionalProp<Signal<T>>
where
    T: Send + Sync + 'static,
{
    fn from(value: ReadSignal<T>) -> Self {
        Self(Some(Signal::from(value)))
    }
}

impl<T> From<RwSignal<T>> for OptionalProp<Signal<T>>
where
    T: Send + Sync + 'static,
{
    fn from(value: RwSignal<T>) -> Self {
        Self(Some(Signal::from(value)))
    }
}

impl<T> From<Memo<T>> for OptionalProp<Signal<T>>
where
    T: Send + Sync + 'static,
{
    fn from(value: Memo<T>) -> Self {
        Self(Some(Signal::from(value)))
    }
}

impl<F, A, Return> From<F> for OptionalProp<BoxOneCallback<A, Return>>
where
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self(Some(BoxOneCallback::new(value)))
    }
}

impl<T> From<Option<T>> for OptionalProp<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use super::OptionalProp;
    use leptos::prelude::Signal;

    #[test]
    fn from() {
        let _prop: OptionalProp<Signal<String>> = "prop".into();
        let _prop: OptionalProp<Signal<String>> = "prop".to_string().into();
        let _prop: OptionalProp<String> = "prop".into();
    }
}
