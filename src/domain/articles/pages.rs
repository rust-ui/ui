use std::fmt::Write as _;

use app_config::{BreadcrumbItem, JsonLdArticle, JsonLdBreadcrumb, SeoMeta, SiteConfig};
use dioxus::prelude::*;
use registry::hooks::use_table_of_contents::use_table_of_contents;

use crate::Route;
use crate::components::app_footer::AppFooter;
use crate::components::navigation::app_header::AppHeader;
use crate::components::table_of_contents::TocItem;
use crate::domain::articles::content::{Article, ArticleCategory, all_articles, article_by_slug};
use crate::markdown::converter::{MdComponents, convert_md, extract_toc_from_md};

#[component]
pub fn ArticlesLayout() -> Element {
    rsx! {
        AppHeader {}
        Outlet::<Route> {}
        AppFooter {}
    }
}

#[component]
pub fn ArticlesPage() -> Element {
    rsx! { ArticlesHubPage { initial_category: None } }
}

#[component]
pub fn ArticleCategoryPage(category: String) -> Element {
    rsx! {
        ArticlesHubPage {
            initial_category: ArticleCategory::from_slug(&category),
        }
    }
}

#[component]
fn ArticlesHubPage(#[props(default)] initial_category: Option<ArticleCategory>) -> Element {
    let mut search = use_signal(String::new);
    let mut selected_category = use_signal(|| initial_category);
    let articles = all_articles();
    let canonical_path = initial_category
        .map_or_else(|| "/articles".to_string(), |category| format!("/articles/category/{}", category.slug()));
    let page_title = initial_category.map_or_else(
        || "Dioxus and Rust UI Articles · Rust/UI".to_string(),
        |category| format!("{} Dioxus and Rust UI Articles · Rust/UI", category.label()),
    );
    let featured = articles.first().copied();
    let query = search().to_lowercase();
    let selected = selected_category();
    let visible = articles
        .iter()
        .copied()
        .filter(|article| selected.is_none_or(|category| article.category == category))
        .filter(|article| {
            query.is_empty()
                || article.title().to_lowercase().contains(&query)
                || article.description().to_lowercase().contains(&query)
                || article.category.label().to_lowercase().contains(&query)
        })
        .collect::<Vec<_>>();

    rsx! {
        SeoMeta {
            title: page_title,
            description: "Practical Dioxus and Rust UI articles covering components, state, forms, responsive design, async data, and shipping cross-platform apps.".to_string(),
            canonical_url: format!("{}{}", SiteConfig::BASE_URL, canonical_path),
        }
        div { class: "mx-auto max-w-[1200px] px-6 pb-24 pt-16 md:pt-24",
            header { class: "mx-auto flex max-w-[680px] flex-col items-center gap-4 text-center",
                BracketHeading { kicker: "Articles", "Dioxus and Rust UI guides" }
                p { class: "mx-auto max-w-[42ch] text-balance text-lg leading-[1.4] text-muted-foreground",
                    "{articles.len()} practical guides across {ArticleCategory::ALL.len()} categories: building, components, interaction, and production."
                }
            }

            if let Some(featured) = featured {
                section { class: "mt-16 grid gap-8 lg:grid-cols-[1.4fr_1fr]",
                    div { class: "rounded-[32px] bg-black/[0.02] p-6 dark:bg-white/[0.03]",
                        PanelLabel { "Featured" }
                        FeaturedArticle { article: featured }
                    }
                    div { class: "rounded-[32px] bg-black/[0.02] p-6 dark:bg-white/[0.03]",
                        PanelLabel { "Spotlight" }
                        div { class: "flex flex-col gap-3",
                            for article in articles.iter().copied().skip(1).take(4) {
                                SpotlightArticle { key: "{article.slug}", article }
                            }
                        }
                    }
                }
            }

            section { class: "mt-16",
                div { class: "flex flex-col gap-5 border-b pb-6 md:flex-row md:items-end md:justify-between",
                    div {
                        PanelLabel { "Browse guides" }
                        h2 { class: "mt-2 text-2xl font-semibold tracking-tight", "Build better Rust UI" }
                    }
                    div { class: "relative w-full md:max-w-[300px]",
                        SearchGlyph {}
                        input {
                            class: "h-10 w-full rounded-full border bg-background pl-10 pr-4 text-sm outline-none transition-colors placeholder:text-muted-foreground focus:border-primary focus:ring-2 focus:ring-primary/20",
                            r#type: "search",
                            placeholder: "Search articles",
                            value: search(),
                            oninput: move |event| search.set(event.value()),
                        }
                    }
                }
                div { class: "mt-6 flex flex-wrap gap-2",
                    button {
                        class: chip_class(selected.is_none()),
                        onclick: move |_| selected_category.set(None),
                        "All guides"
                    }
                    for category in ArticleCategory::ALL {
                        {
                            let is_selected = selected == Some(category);
                            let count = articles.iter().filter(|article| article.category == category).count();
                            rsx! {
                                button {
                                    class: chip_class(is_selected),
                                    onclick: move |_| selected_category.set(Some(category)),
                                    "{category.label()}"
                                    span { class: "ml-1 text-[11px] opacity-60", "{count}" }
                                }
                            }
                        }
                    }
                }
                if visible.is_empty() {
                    div { class: "mt-10 rounded-2xl border border-dashed p-10 text-center text-sm text-muted-foreground",
                        "No articles match that filter."
                    }
                } else {
                    div { class: "mt-8 grid gap-5 md:grid-cols-2 lg:grid-cols-3",
                        for article in visible {
                            ArticleCard { key: "{article.slug}", article }
                        }
                    }
                }
            }

            CategoryCarousel {}
            ArticleClosingCta {}
        }
    }
}

#[component]
pub fn ArticlePage(slug: String) -> Element {
    let Some(article) = article_by_slug(&slug) else {
        return rsx! {
            div { class: "mx-auto max-w-[1200px] px-6 py-24 text-center",
                h1 { class: "text-3xl font-semibold", "Article not found" }
                Link { to: Route::ArticlesPage {}, class: "mt-4 inline-block underline underline-offset-4", "Back to articles" }
            }
        };
    };

    let fm = article.frontmatter();
    let canonical_url = format!("{}{}/{}", SiteConfig::BASE_URL, "/articles", article.slug);
    let toc_items = extract_toc_from_md(article.body());
    let components = MdComponents::new();
    let body = convert_md(article.body(), &components);
    let author = if fm.author.is_empty() { "Max Wells" } else { &fm.author };
    let author_role = if fm.author_role.is_empty() { "Creator of Rust/UI and Rustify" } else { &fm.author_role };
    let author_image = if fm.author_image.is_empty() { "/articles/author-max-wells.webp" } else { &fm.author_image };
    let publish_date = if fm.publish_date.is_empty() { "2026-09-22" } else { &fm.publish_date };

    rsx! {
        SeoMeta {
            key: "{article.slug}",
            title: format!("{} · Rust/UI", article.title()),
            description: article.description(),
            canonical_url: canonical_url.clone(),
            image_url: Some(format!("{}/og-image.webp", SiteConfig::BASE_URL)),
            og_title: Some(article.title()),
            og_type: Some("article".to_string()),
        }
        JsonLdArticle {
            title: article.title(),
            description: article.description(),
            url: canonical_url.clone(),
            date_published: Some(publish_date.to_string()),
            date_modified: (!fm.last_updated.is_empty()).then_some(fm.last_updated.clone()),
            keywords: fm.keywords,
            article_section: article.category.label().to_string(),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Articles".to_string(), url: Some(format!("{}/articles", SiteConfig::BASE_URL)) },
                BreadcrumbItem { name: article.category.label().to_string(), url: Some(format!("{}/articles/category/{}", SiteConfig::BASE_URL, article.category.slug())) },
                BreadcrumbItem { name: article.title(), url: None },
            ],
        }
        div { class: "mx-auto max-w-[1200px] px-6 pb-24 pt-16 md:pt-24",
            header { class: "grid gap-10 md:grid-cols-[minmax(0,1fr)_auto] md:items-start",
                div { class: "flex flex-col gap-6",
                    nav { class: "flex items-center gap-2", "aria-label": "Breadcrumb",
                        Link { to: Route::ArticlesPage {}, class: "rounded-full border bg-black/[0.04] px-2.5 py-1 text-[11px] font-medium uppercase dark:bg-white/[0.06]", "Articles" }
                        span { class: "text-sm text-muted-foreground", "/" }
                        Link { to: Route::ArticleCategoryPage { category: article.category.slug().to_string() }, class: "rounded-full border border-primary/30 bg-primary/10 px-2.5 py-1 text-[11px] font-medium uppercase text-primary", "{article.category.label()}" }
                    }
                    h1 { class: "text-[clamp(38px,5vw,52px)] font-semibold leading-[1.1] tracking-[-0.042em]", "{article.title()}" }
                    div { class: "flex w-fit items-center gap-3",
                        img {
                            src: "{author_image}",
                            alt: "{author}",
                            width: "44",
                            height: "44",
                            class: "size-11 rounded-full object-cover",
                        }
                        span { class: "flex flex-col",
                            span { class: "text-sm font-medium", "{author}" }
                            span { class: "text-xs text-muted-foreground", "{author_role}" }
                        }
                    }
                }
                ArticleThumbnail { article, feature: true }
            }
            div { class: "mt-16 grid gap-12 lg:grid-cols-[262px_minmax(0,1fr)]",
                ArticleSidebar {
                    toc_items,
                    share_url: canonical_url,
                    share_title: article.title(),
                }
                article { class: "min-w-0",
                    div { class: "mb-8 flex flex-wrap gap-3 text-sm text-muted-foreground",
                        time { datetime: "{publish_date}", "{publish_date}" }
                        span { aria_hidden: "true", "·" }
                        span { "{article.reading_time()} min read" }
                        span { aria_hidden: "true", "·" }
                        span { "{article.description()}" }
                    }
                    div { class: "max-w-none text-[16px] leading-7", {body} }
                    div { class: "mt-16 rounded-2xl border bg-muted/30 p-6",
                        p { class: "font-medium", "Keep building with Rust/UI" }
                        p { class: "mt-2 text-sm text-muted-foreground",
                            "Browse copyable Dioxus components and Rust UI patterns for your next application."
                        }
                        Link { to: Route::DocsComponentsIndexPage {}, class: "mt-4 inline-block text-sm font-medium underline underline-offset-4", "Browse components →" }
                    }
                }
            }
            ArticleClosingCta {}
        }
    }
}

#[component]
fn BracketHeading(#[props(default)] kicker: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-3",
            p { class: "text-xs font-medium uppercase tracking-[0.16em] text-primary", "[ {kicker} ]" }
            h1 { class: "text-4xl font-semibold tracking-[-0.04em] md:text-5xl", {children} }
        }
    }
}

#[component]
fn PanelLabel(children: Element) -> Element {
    rsx! { p { class: "mb-5 text-[11px] font-medium uppercase tracking-[0.16em] text-muted-foreground", {children} } }
}

#[component]
fn FeaturedArticle(article: Article) -> Element {
    rsx! {
        Link { to: Route::ArticlePage { slug: article.slug.to_string() }, class: "group block",
            ArticleThumbnail { article, feature: true }
            div { class: "mt-6 flex items-center gap-2 text-xs font-medium uppercase text-primary",
                "{article.category.label()}"
                span { class: "text-muted-foreground", "· {article.reading_time()} min" }
            }
            h2 { class: "mt-3 text-2xl font-semibold leading-tight tracking-tight group-hover:underline", "{article.title()}" }
            p { class: "mt-3 line-clamp-3 text-sm leading-6 text-muted-foreground", "{article.description()}" }
        }
    }
}

#[component]
fn SpotlightArticle(article: Article) -> Element {
    rsx! {
        Link { to: Route::ArticlePage { slug: article.slug.to_string() }, class: "group flex gap-4 rounded-2xl p-2 transition-colors hover:bg-background",
            ArticleThumbnail { article, feature: false }
            div { class: "min-w-0 py-1",
                p { class: "text-[10px] font-medium uppercase tracking-[0.12em] text-primary", "{article.category.label()}" }
                h3 { class: "mt-1 line-clamp-2 text-sm font-semibold leading-5 group-hover:underline", "{article.title()}" }
                p { class: "mt-1 text-xs text-muted-foreground", "{article.reading_time()} min read" }
            }
        }
    }
}

#[component]
fn ArticleCard(article: Article) -> Element {
    rsx! {
        Link { to: Route::ArticlePage { slug: article.slug.to_string() }, class: "group flex min-w-0 flex-col overflow-hidden rounded-2xl border bg-card transition-all hover:-translate-y-0.5 hover:shadow-lg",
            ArticleThumbnail { article, feature: false }
            div { class: "flex flex-1 flex-col p-5",
                p { class: "text-[10px] font-medium uppercase tracking-[0.12em] text-primary", "{article.category.label()}" }
                h3 { class: "mt-2 text-lg font-semibold leading-tight tracking-tight group-hover:underline", "{article.title()}" }
                p { class: "mt-3 line-clamp-3 text-sm leading-6 text-muted-foreground", "{article.description()}" }
                div { class: "mt-auto pt-5 text-xs text-muted-foreground", "{article.reading_time()} min read · Rust/UI" }
            }
        }
    }
}

#[component]
fn ArticleThumbnail(article: Article, #[props(default)] feature: bool) -> Element {
    let size_class = if feature { "aspect-[1200/675]" } else { "aspect-[16/9]" };
    let accent = match article.category {
        ArticleCategory::Foundations => "from-orange-500/80 via-amber-400/30 to-background",
        ArticleCategory::Components => "from-sky-500/80 via-cyan-400/30 to-background",
        ArticleCategory::Interaction => "from-violet-500/80 via-fuchsia-400/30 to-background",
        ArticleCategory::Production => "from-emerald-500/80 via-teal-400/30 to-background",
    };
    rsx! {
        div { class: "relative {size_class} w-full overflow-hidden rounded-2xl bg-black",
            div { class: "absolute inset-0 bg-gradient-to-br {accent} opacity-90" }
            div { class: "absolute inset-0 bg-[radial-gradient(circle_at_80%_20%,white_0,transparent_28%)] opacity-20" }
            div { class: "absolute inset-x-0 bottom-0 p-5 text-white",
                p { class: "text-[10px] font-medium uppercase tracking-[0.16em] text-white/70", "Dioxus · Rust UI" }
                p { class: "mt-2 max-w-[26ch] text-lg font-semibold leading-tight tracking-tight", "{article.short_title()}" }
            }
        }
    }
}

#[component]
fn CategoryCarousel() -> Element {
    rsx! {
        section { class: "mt-20",
            PanelLabel { "Explore by category" }
            div { class: "grid gap-3 sm:grid-cols-2 lg:grid-cols-4",
                for category in ArticleCategory::ALL {
                    Link { key: "{category.slug()}", to: Route::ArticleCategoryPage { category: category.slug().to_string() }, class: "group rounded-2xl border p-5 transition-colors hover:bg-muted",
                        div { class: "flex items-center justify-between",
                            h3 { class: "font-semibold group-hover:underline", "{category.label()}" }
                            span { class: "text-primary", "→" }
                        }
                        p { class: "mt-2 text-sm leading-6 text-muted-foreground", "{category.blurb()}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleClosingCta() -> Element {
    rsx! {
        section { class: "mt-20 rounded-[28px] border bg-muted/30 p-8 text-center md:p-12",
            p { class: "text-xs font-medium uppercase tracking-[0.16em] text-primary", "Keep building" }
            h2 { class: "mt-3 text-2xl font-semibold tracking-tight md:text-3xl", "Turn the guide into a real Rust UI" }
            p { class: "mx-auto mt-3 max-w-xl text-sm leading-6 text-muted-foreground",
                "Use the Rust/UI registry as a starting point, then copy, adapt, and own the components in your Dioxus application."
            }
            Link { to: Route::DocsComponentsIndexPage {}, class: "mt-6 inline-flex rounded-full bg-primary px-5 py-2.5 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90", "Browse components →" }
        }
    }
}

#[component]
fn ArticleSidebar(toc_items: Vec<TocItem>, share_url: String, share_title: String) -> Element {
    let anchors = toc_items.iter().map(|item| item.anchor.clone()).collect::<Vec<_>>();
    let toc_state = use_table_of_contents(&anchors);
    let active_anchor = toc_state.active_anchor;
    let mut copied = use_signal(|| false);
    let copy_url = share_url.clone();

    rsx! {
        aside { class: "flex h-max flex-col gap-8 lg:sticky lg:top-[122px]",
            div { class: "flex flex-col gap-3",
                p { class: "text-[12px] font-medium uppercase tracking-[0.04em] text-muted-foreground", "Table of contents" }
                ul { class: "flex flex-col gap-1.5",
                    for item in toc_items {
                        {
                            let anchor = item.anchor.clone();
                            let is_current = active_anchor().as_deref() == Some(anchor.as_str());
                            rsx! {
                                li { key: "{anchor}", class: "min-w-0",
                                    a {
                                        href: "#{anchor}",
                                        "aria-current": if is_current { "true" } else { "false" },
                                        title: "{item.title}",
                                        class: if is_current { "block truncate rounded-[7.78px] bg-muted p-[5.83px] text-sm text-foreground transition-colors" } else { "block truncate rounded-[7.78px] p-[5.83px] text-sm text-muted-foreground transition-colors hover:bg-muted hover:text-foreground" },
                                        onclick: move |event| {
                                            event.prevent_default();
                                            #[cfg(target_arch = "wasm32")]
                                            if let Some(window) = web_sys::window()
                                                && let Some(document) = window.document()
                                                && let Some(element) = document.get_element_by_id(&anchor)
                                            {
                                                let options = web_sys::ScrollIntoViewOptions::new();
                                                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                                                options.set_block(web_sys::ScrollLogicalPosition::Start);
                                                element.scroll_into_view_with_scroll_into_view_options(&options);
                                            }
                                        },
                                        "{item.title}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col gap-3 border-t pt-6",
                p { class: "text-[12px] font-medium uppercase tracking-[0.04em] text-muted-foreground", "Share article" }
                div { class: "flex gap-2",
                    button {
                        type: "button",
                        "aria-label": if copied() { "Copied" } else { "Copy to clipboard" },
                        class: if copied() { "flex size-9 items-center justify-center rounded-full border bg-green-100 text-green-700 transition-colors" } else { "flex size-9 items-center justify-center rounded-full border bg-background text-muted-foreground transition-colors hover:bg-muted" },
                        onclick: move |_| {
                            let value = copy_url.clone();
                            spawn(async move {
                                let js = format!("navigator.clipboard?.writeText({value:?})");
                                let _ = dioxus::document::eval(&js).await;
                                copied.set(true);
                            });
                        },
                        if copied() { CheckGlyph {} } else { LinkGlyph {} }
                    }
                    ShareLink { label: "X", href: share_href("https://x.com/intent/post?text=", &share_title, &share_url) }
                    ShareLink { label: "WA", href: share_href("https://wa.me/?text=", &share_title, &share_url) }
                    ShareLink { label: "in", href: format!("https://www.linkedin.com/sharing/share-offsite/?url={}", encode_component(&share_url)) }
                }
            }
        }
    }
}

#[component]
fn ShareLink(label: &'static str, href: String) -> Element {
    rsx! {
        a {
            href: "{href}",
            target: "_blank",
            rel: "noopener noreferrer",
            "aria-label": "Share on {label}",
            class: "flex size-9 items-center justify-center rounded-full border bg-background text-[10px] font-semibold text-muted-foreground transition-colors hover:bg-muted",
            "{label}"
        }
    }
}

fn share_href(prefix: &str, title: &str, url: &str) -> String {
    format!("{}{}%20{}", prefix, encode_component(title), encode_component(url))
}

fn encode_component(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}

const fn chip_class(selected: bool) -> &'static str {
    if selected {
        "rounded-full border border-primary bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground"
    } else {
        "rounded-full border bg-background px-3 py-1.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
    }
}

#[component]
fn SearchGlyph() -> Element {
    rsx! {
        svg { class: "pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", "aria-hidden": "true",
            circle { cx: "11", cy: "11", r: "8" }
            path { d: "m21 21-4.3-4.3" }
        }
    }
}

#[component]
fn LinkGlyph() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", class: "size-[19px]", "aria-hidden": "true",
            path { d: "M9 15 15 9M10.5 6.5l.8-.9a4 4 0 0 1 5.7 5.7l-.9.8M13.5 17.5l-.8.9a4 4 0 0 1-5.7-5.7l.9-.8", stroke: "currentColor", stroke_width: "1.7", stroke_linecap: "round" }
        }
    }
}

#[component]
fn CheckGlyph() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", fill: "none", class: "size-[19px]", "aria-hidden": "true",
            path { d: "m5 12.5 4.5 4.5L19 7", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round" }
        }
    }
}
