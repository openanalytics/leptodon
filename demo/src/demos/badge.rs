use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_components::badge::Badge;
use leptos_components::badge::BadgePrefix;
use leptos_components::badge::BadgeSize;
use leptos_components::badge::BadgeTheme;
use leptos_components::heading::Heading4;
use leptos_components::icon::WarningIcon;
use leptos_components::layout::FixedCenterColumn;
use leptos_meta::Title;

#[generate_codeblock(PlainBadgesExample)]
#[component]
pub fn PlainBadgesDemo() -> impl IntoView {
    view! {
        <Badge>Plain</Badge>
    }
}

#[generate_codeblock(PrefixedBadgesExample)]
#[component]
pub fn PrefixedBadgesDemo() -> impl IntoView {
    view! {
        <div class="flex gap-1">
            <Badge theme=BadgeTheme::Brand prefix=BadgePrefix::Avatar { src: "https://avatars.githubusercontent.com/u/274806".to_string() }>Brand</Badge>
            <Badge theme=BadgeTheme::Danger prefix=BadgePrefix::Dot>Danger</Badge>
            <Badge theme=BadgeTheme::Warning prefix=BadgePrefix::Icon(WarningIcon())>Warning</Badge>
            <Badge theme=BadgeTheme::Secondary prefix=BadgePrefix::SvgLoader>Secondary</Badge>
            <Badge theme=BadgeTheme::Success prefix=BadgePrefix::Dot>Success</Badge>
            <Badge theme=BadgeTheme::Transparent prefix=BadgePrefix::Dot>Transparent</Badge>
        </div>
    }
}

#[generate_codeblock(DissmissableBadgesExample)]
#[component]
pub fn DismissableBadgesDemo() -> impl IntoView {
    view! {
        <div class="flex gap-1">
            <Badge theme=BadgeTheme::Brand dismissable=true>Brand</Badge>
            <Badge theme=BadgeTheme::Danger dismissable=true>Danger</Badge>
            <Badge theme=BadgeTheme::Warning dismissable=true>Warning</Badge>
            <Badge theme=BadgeTheme::Secondary dismissable=true>Secondary</Badge>
            <Badge theme=BadgeTheme::Success dismissable=true>Success</Badge>
            <Badge theme=BadgeTheme::Transparent dismissable=true>Transparent</Badge>
        </div>
    }
}

#[generate_codeblock(LargeBorderedBadgesExample)]
#[component]
pub fn LargeBorderedBadgesDemo() -> impl IntoView {
    view! {
        <div class="flex gap-1">
            <Badge theme=BadgeTheme::Brand size=BadgeSize::Large border=true>Brand</Badge>
            <Badge theme=BadgeTheme::Danger size=BadgeSize::Large border=true>Danger</Badge>
            <Badge theme=BadgeTheme::Warning size=BadgeSize::Large border=true>Warning</Badge>
            <Badge theme=BadgeTheme::Secondary size=BadgeSize::Large border=true>Secondary</Badge>
            <Badge theme=BadgeTheme::Success size=BadgeSize::Large border=true>Success</Badge>
            <Badge theme=BadgeTheme::Transparent size=BadgeSize::Large border=true>Transparent</Badge>
        </div>
    }
}

#[component]
pub fn BadgeDemoPage() -> impl IntoView {
    view! {
        <Title text="Badge Component"/>

        <FixedCenterColumn>
            <Heading4 anchor="plain">"Plain Badge"</Heading4>
            <PlainBadgesExample />

            <Heading4 anchor="prefixed-badges">"Prefixed Badges"</Heading4>
            <PrefixedBadgesExample />

            <Heading4 anchor="dissmissable-badges">"Dissmissable Badges"</Heading4>
            <DissmissableBadgesExample />

            <Heading4 anchor="large-bordered-badges">"Large Bordered Badges"</Heading4>
            <LargeBorderedBadgesExample />

            <leptos_components::badge::BadgeDocs />
        </FixedCenterColumn>
    }
}
