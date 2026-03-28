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
use leptos::{either::EitherOf3, prelude::*};

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}

#[component]
pub fn OptionComp<T: 'static, CF, IV>(
    value: Option<T>,
    children: CF,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView
where
    CF: FnOnce(T) -> IV + 'static,
    IV: IntoView + 'static,
{
    if let Some(value) = value {
        EitherOf3::A(children(value))
    } else if let Some(fallback) = fallback {
        EitherOf3::B((fallback.children)())
    } else {
        EitherOf3::C(())
    }
}
