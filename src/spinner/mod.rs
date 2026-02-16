use attr_docgen::generate_docs;
use leptos::prelude::*;

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;

#[derive(Default)]
pub enum SpinnerAppearance {
    #[default]
    Default,
    OA,
    Custom(MaybeReactiveClass),
}

impl SpinnerAppearance {
    fn into_class(self) -> MaybeReactiveClass {
        match self {
            SpinnerAppearance::Default => {
                "text-gray-900 stroke-oa-gray dark:text-oa-gray dark:stroke-gray-700".into()
            }
            SpinnerAppearance::OA => {
                "text-oa-blue stroke-oa-gray dark:text-oa-blue dark:stroke-gray-700".into()
            }
            SpinnerAppearance::Custom(class) => class,
        }
    }
}

#[generate_docs]
#[component]
pub fn Spinner(
    /// Extra style classes. E.g. change size.
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// Spinner appearance
    #[prop(optional)]
    appearance: SpinnerAppearance,
) -> impl IntoView {
    view! {
        <div
            role="progressbar"
        >
            <svg
               width="35.174278mm"
               height="35.174034mm"
               viewBox="0 0 35.174278 35.174034"
               version="1.1"
               class=class_list!("animate-spin ", class, appearance.into_class())
               id="svg1"
               xmlns="http://www.w3.org/2000/svg"
               xmlns:svg="http://www.w3.org/2000/svg">
              <defs
                 id="defs1" />
              <g
                 id="layer1"
                 transform="translate(-49.206963,-49.206252)">
                <path
                    style="fill:none;stroke-width:3.175;stroke-linecap:round;stroke-dasharray:none;stroke-opacity:1"
                    id="path1-3"
                    stroke="currentFill"
                    d="m -50.793747,66.793747 a 16,16 0 0 1 -16,16"
                    transform="rotate(-90)" />
                <path
                   style="fill:none;stroke-width:3.175;stroke-linecap:round;stroke-dasharray:none;stroke-opacity:1"
                   id="path1"
                   stroke="currentColor"
                   d="M 82.793495,66.793747 A 16,16 0 0 1 72.91643,81.575819 16,16 0 0 1 55.479787,78.107455 16,16 0 0 1 52.011423,60.670812 16,16 0 0 1 66.793495,50.793747" />
              </g>
            </svg>
        </div>
    }
}
