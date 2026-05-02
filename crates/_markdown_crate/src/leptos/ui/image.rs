use leptos::prelude::*;

use crate::leptos::leptos_converter::MdComponentProps;

/// Usage in markdown:
/// <Image src="/images/foo.png" alt="Description" />
/// <Image src="/images/foo.png" alt="Description" width="200" />
/// <Image src="/images/foo.png" alt="Description" class="rounded-full w-32 mx-auto" />
pub fn image_md(props: MdComponentProps) -> impl IntoView {
    let src = props.attributes.get("src").and_then(|v| v.clone()).unwrap_or_default();
    let alt = props.attributes.get("alt").and_then(|v| v.clone()).unwrap_or_default();
    let width = props.attributes.get("width").and_then(|v| v.clone());
    let class = props
        .attributes
        .get("class")
        .and_then(|v| v.clone())
        .unwrap_or_else(|| "max-w-full".to_string());

    view! {
        <img src=src alt=alt width=width class=class loading="lazy" decoding="async" />
    }
}
