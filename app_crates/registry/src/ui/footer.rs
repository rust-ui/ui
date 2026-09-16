use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Footer(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("", class.as_deref().unwrap_or(""));
    rsx! { footer { "data-name": "Footer", role: "contentinfo", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterBrandLink(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] href: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!("block size-fit", class.as_deref().unwrap_or(""));
    rsx! {
        Link {
            "data-name": "FooterBrandLink",
            class: "{merged_class}",
            to: href.as_deref().unwrap_or("/"),
            "aria-label": aria_label.as_deref().unwrap_or("go home"),
            {children}
        }
    }
}

#[component]
pub fn FooterLinksSection(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("space-y-4 text-sm", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FooterLinksSection", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("block font-medium", class.as_deref().unwrap_or(""));
    rsx! { span { "data-name": "FooterTitle", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterLink(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] href: Option<String>,
    children: Element,
) -> Element {
    let merged_class =
        tw_merge!("block duration-150 text-foreground/70 hover:text-primary", class.as_deref().unwrap_or(""));
    rsx! {
        Link {
            "data-name": "FooterLink",
            class: "{merged_class}",
            to: href.as_deref().unwrap_or("#"),
            {children}
        }
    }
}

#[component]
pub fn FooterLinks(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("flex flex-wrap gap-4 sm:flex-col", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FooterLinks", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("text-sm text-foreground/70 text-balance", class.as_deref().unwrap_or(""));
    rsx! { p { "data-name": "FooterDescription", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterGrid(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("grid gap-12 md:grid-cols-5", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FooterGrid", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterContainer(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("px-6 mx-auto max-w-5xl", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FooterContainer", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterSection(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "w-full max-w-5xl mx-auto py-6 flex flex-wrap gap-4 justify-between items-center [.border-b]:mb-14 [.border-t]:mt-14",
        class.as_deref().unwrap_or("")
    );
    rsx! { section { "data-name": "FooterSection", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterSocialContainer(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("flex flex-wrap gap-6 text-sm", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FooterSocialContainer", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterBrand(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("md:col-span-2", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FooterBrand", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterSectionsGrid(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("grid gap-6", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FooterSectionsGrid", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterCopyright(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("text-sm text-foreground/70", class.as_deref().unwrap_or(""));
    rsx! { small { "data-name": "FooterCopyright", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterNavContainer(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("flex flex-wrap gap-6 justify-center my-8 text-sm", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "FooterNavContainer", class: "{merged_class}", {children} } }
}

#[component]
pub fn FooterExternalLink(
    #[props(into, optional)] class: Option<String>,
    #[props(into)] href: String,
    #[props(into, optional)] aria_label: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!("block text-foreground/70 hover:text-primary", class.as_deref().unwrap_or(""));
    rsx! {
        a {
            "data-name": "FooterExternalLink",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "{merged_class}",
            href: "{href}",
            aria_label: aria_label.as_deref().unwrap_or("Social media link"),
            {children}
        }
    }
}
