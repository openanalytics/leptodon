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
