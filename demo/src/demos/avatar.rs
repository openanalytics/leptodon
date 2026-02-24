use attr_docgen::generate_codeblock;
use leptodon::avatar::Avatar;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(AvatarPlaceholderExample)]
#[component]
pub fn AvatarPlaceholderDemo() -> impl IntoView {
    view! {
        <Avatar/>
    }
}

#[generate_codeblock(AvatarLinkExample)]
#[component]
pub fn AvatarLinkDemo() -> impl IntoView {
    view! {
        <Avatar src="https://avatars.githubusercontent.com/u/274806"/>
    }
}

#[component]
pub fn AvatarDemoPage() -> impl IntoView {
    view! {
        <Title text="Avatar Component"/>

        <FixedCenterColumn>
            <Heading4 anchor="avatar-placeholder">"Avatar Placeholder"</Heading4>
            <AvatarPlaceholderExample />

            <Heading4 anchor="avatar">"Avatar Link"</Heading4>
            <AvatarLinkExample />

            <leptodon::avatar::AvatarDocs />
        </FixedCenterColumn>
    }
}
