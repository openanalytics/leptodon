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
use leptos::{
    oco::Oco,
    prelude::{Get, MaybeProp, Memo},
};

use crate::class_list::{Class, ClassList, IntoClass};

/// The ClassList loses its reactive tracking when used in non reactive contexts.
/// Thus this type exists to avoid having to create closures every time a class is used.
///   Quickly wrap ClassList or &'static str as a ReactiveClass via .into()
///   Pass the reactive_class to class_list!(reactive_class, ...) in the child component.
#[derive(Clone)]
pub enum ReactiveClass {
    Memo(Memo<Oco<'static, str>>),
    Static(Oco<'static, str>),
}

impl IntoClass for ReactiveClass {
    fn into_class(self) -> Class {
        match self {
            ReactiveClass::Memo(memo) => Class::FnString(Box::new(move || memo.get())),
            ReactiveClass::Static(oco) => Class::String(oco),
        }
    }
}

impl IntoClass for MaybeProp<ReactiveClass> {
    fn into_class(self) -> Class {
        Class::FnOptionString(Box::new(move || match self.get() {
            Some(ReactiveClass::Memo(memo)) => Some(memo.get()),
            Some(ReactiveClass::Static(oco)) => Some(oco),
            None => None,
        }))
    }
}

impl From<ClassList> for ReactiveClass {
    fn from(class_list: ClassList) -> Self {
        ReactiveClass::Memo(Memo::new(move |_old| {
            let mut class = String::new();
            class_list.write_class_string(&mut class);
            Oco::Owned(class)
        }))
    }
}

impl From<&'static str> for ReactiveClass {
    fn from(value: &'static str) -> Self {
        ReactiveClass::Static(value.into())
    }
}

impl From<String> for ReactiveClass {
    fn from(value: String) -> Self {
        ReactiveClass::Static(value.into())
    }
}

/// MaybeProp variant of [ReactiveClass]. See [ReactiveClass]
#[derive(Default)]
pub struct MaybeReactiveClass(MaybeProp<ReactiveClass>);

impl From<&'static str> for MaybeReactiveClass {
    fn from(value: &'static str) -> Self {
        MaybeReactiveClass(MaybeProp::derive(move || {
            Some(ReactiveClass::Static(value.into()))
        }))
    }
}

impl From<ClassList> for MaybeReactiveClass {
    fn from(value: ClassList) -> Self {
        let memo = ReactiveClass::from(value);
        MaybeReactiveClass(MaybeProp::derive(move || Some(memo.clone())))
    }
}

impl From<MaybeProp<String>> for MaybeReactiveClass {
    fn from(value: MaybeProp<String>) -> Self {
        MaybeReactiveClass(MaybeProp::derive(move || {
            value.get().map(ReactiveClass::from)
        }))
    }
}

impl IntoClass for MaybeReactiveClass {
    fn into_class(self) -> Class {
        self.0.into_class()
    }
}
