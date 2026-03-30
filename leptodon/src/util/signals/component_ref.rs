// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
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
// MIT License, taken from https://github.com/thaw-ui/thaw at 69bc65d
use leptos::{
    logging::debug_warn,
    prelude::{
        ArcReadSignal, ArcRwSignal, ArcWriteSignal, Get, GetUntracked, RwSignal, Storage,
        SyncStorage, Update,
    },
};

pub struct ComponentRef<T, S = SyncStorage>(RwSignal<Option<T>, S>);

impl<T> Default for ComponentRef<T>
where
    T: Send + Sync + 'static,
{
    fn default() -> Self {
        Self(RwSignal::new(None))
    }
}

impl<T, S> Clone for ComponentRef<T, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, S> Copy for ComponentRef<T, S> {}

impl<T> ComponentRef<T>
where
    T: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, S> ComponentRef<T, S>
where
    T: Clone + 'static,
    S: Storage<ArcRwSignal<Option<T>>> + Storage<ArcReadSignal<Option<T>>>,
{
    pub fn get(&self) -> Option<T> {
        self.0.get()
    }

    pub fn try_get(&self) -> Option<T> {
        self.0.try_get().flatten()
    }

    pub fn get_untracked(&self) -> Option<T> {
        self.0.get_untracked()
    }

    pub fn try_get_untracked(&self) -> Option<T> {
        self.0.try_get_untracked().flatten()
    }
}

impl<T, S> ComponentRef<T, S>
where
    T: 'static,
    S: Storage<ArcRwSignal<Option<T>>> + Storage<ArcWriteSignal<Option<T>>>,
{
    pub fn load(&self, comp: T) {
        self.0.update(|current| {
            if current.is_some() {
                debug_warn!(
                    "You are setting a ComponentRef that has already been filled. \
                     It’s possible this is intentional."
                );
            }
            *current = Some(comp);
        });
    }

    // pub fn on_load<F>(self, f: F)
    // where
    //     T: Clone,
    //     F: FnOnce(T) + 'static,
    // {
    //     let f = Cell::new(Some(f));

    //     RenderEffect::new(move |_| {
    //         if let Some(comp) = self.get() {
    //             f.take().unwrap()(comp);
    //         }
    //     });
    // }
}
