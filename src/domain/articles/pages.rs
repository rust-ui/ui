use std::fmt::Write as _;

use app_config::{BreadcrumbItem, JsonLdArticle, JsonLdBreadcrumb, SeoMeta, SiteConfig};
use dioxus::prelude::*;
use icons::{ArrowLeft, ArrowRight, BookOpen, Boxes, Cog, Rocket};
use registry::hooks::use_table_of_contents::use_table_of_contents;

use crate::Route;
use crate::components::app_footer::AppFooter;
use crate::components::navigation::app_header::AppHeader;
use crate::components::table_of_contents::TocItem;
use crate::domain::articles::content::{Article, ArticleCategory, all_articles, article_by_slug};
use crate::domain::articles::hooks::use_horizontal_rail;
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
    let articles = all_articles();
    let canonical_path = initial_category
        .map_or_else(|| "/articles".to_string(), |category| format!("/articles/category/{}", category.slug()));
    let page_title = initial_category.map_or_else(
        || "Dioxus and Rust UI Articles · Rust/UI".to_string(),
        |category| format!("{} Dioxus and Rust UI Articles · Rust/UI", category.label()),
    );
    let featured = articles.first().copied();
    rsx! {
        SeoMeta {
            title: page_title,
            description: "Practical Dioxus and Rust UI articles covering components, state, forms, responsive design, async data, and shipping cross-platform apps.".to_string(),
            canonical_url: format!("{}{}", SiteConfig::BASE_URL, canonical_path),
        }
        div { class: "mx-auto max-w-[1200px] overflow-x-clip px-6 pb-24 pt-16 md:pt-24",
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

            ArticleListing { articles, initial_category }

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
                ArticleThumbnail { article, feature: true, hero: true }
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
    rsx! { p { class: "pb-2 text-[12px] font-medium uppercase tracking-[-0.01em] text-[rgba(15,15,16,0.5)]", {children} } }
}

#[component]
fn FeaturedArticle(article: Article) -> Element {
    rsx! {
        Link { to: Route::ArticlePage { slug: article.slug.to_string() }, class: "group block rounded-[20px] border border-black/[0.05] bg-white p-2 shadow-md transition-transform duration-200 hover:-translate-y-0.5",
            ArticleThumbnail { article, feature: true, hero: false }
            div { class: "grid gap-4 p-4 sm:grid-cols-2",
                div { class: "flex flex-col gap-2",
                    span { class: "w-fit rounded-[7px] border border-primary/30 bg-primary/10 px-1.5 py-px text-[12px] font-medium uppercase leading-[1.4] tracking-[-0.01em] text-primary", "{article.category.label()}" }
                    h2 { class: "text-[28px] font-semibold leading-[1.1] tracking-[-0.042em] text-[#0f0f10] group-hover:underline", "{article.title()}" }
                }
                p { class: "line-clamp-4 self-center text-[15px] leading-[1.5] text-[rgba(15,15,16,0.73)]", "{article.description()}" }
            }
        }
    }
}

#[component]
fn SpotlightArticle(article: Article) -> Element {
    rsx! {
        Link { to: Route::ArticlePage { slug: article.slug.to_string() }, class: "group grid grid-cols-[128px_minmax(0,1fr)] items-stretch gap-0 rounded-[20px] border border-black/[0.05] bg-white p-2 shadow-md transition-transform duration-200 hover:-translate-y-0.5",
            ArticleThumbnail { article, feature: false }
            div { class: "flex flex-col justify-start gap-1.5 pl-4 pr-2 pt-1",
                span { class: "w-fit rounded-[7px] border border-primary/30 bg-primary/10 px-1.5 py-px text-[11px] font-medium uppercase leading-[1.4] tracking-[-0.01em] text-primary", "{article.category.label()}" }
                h2 { class: "line-clamp-2 text-[15px] font-semibold leading-[1.3] tracking-[-0.03em] text-[#0f0f10] group-hover:underline", "{article.title()}" }
            }
        }
    }
}

#[component]
fn ArticleCard(article: Article) -> Element {
    rsx! {
        Link { to: Route::ArticlePage { slug: article.slug.to_string() }, class: "group block rounded-[20px] border border-black/[0.05] bg-white p-2 shadow-md transition-transform duration-200 hover:-translate-y-0.5",
            ArticleThumbnail { article, feature: true, compact: true, hero: false }
            div { class: "flex flex-col gap-2 px-3 pb-3 pt-4",
                span { class: "w-fit rounded-[7px] border border-primary/30 bg-primary/10 px-1.5 py-px text-[12px] font-medium uppercase leading-[1.4] tracking-[-0.01em] text-primary", "{article.category.label()}" }
                h2 { class: "line-clamp-3 text-[20px] font-semibold leading-[1.1] tracking-[-0.042em] text-[#0f0f10] group-hover:underline", "{article.title()}" }
                p { class: "line-clamp-2 text-[15px] leading-[1.45] text-[rgba(15,15,16,0.73)]", "{article.description()}" }
            }
        }
    }
}

#[component]
fn ArticleListing(articles: &'static [Article], initial_category: Option<ArticleCategory>) -> Element {
    let mut query = use_signal(String::new);
    let mut selected = use_signal(|| initial_category.into_iter().collect::<Vec<_>>());
    let current_query = query().trim().to_lowercase();
    let current_selected = selected();
    let filtered = articles
        .iter()
        .copied()
        .filter(|article| current_selected.is_empty() || current_selected.contains(&article.category))
        .filter(|article| {
            current_query.is_empty()
                || article.title().to_lowercase().contains(&current_query)
                || article.description().to_lowercase().contains(&current_query)
        })
        .collect::<Vec<_>>();
    let is_filtering = !current_query.is_empty() || !current_selected.is_empty();

    rsx! {
        div { class: "mt-16 grid gap-8 lg:grid-cols-[285px_minmax(0,1fr)]",
            aside { class: "flex h-max flex-col gap-6 lg:sticky lg:top-[122px]",
                div { class: "relative flex items-center rounded-[16px] border border-black/[0.08] bg-white shadow-sm",
                    SearchGlyph {}
                    input {
                        r#type: "search",
                        value: query(),
                        placeholder: "Search articles",
                        class: "h-[42px] w-full rounded-[16px] bg-transparent pl-[38px] pr-4 text-[14px] text-[#0f0f10] outline-none placeholder:text-[rgba(15,15,16,0.4)]",
                        oninput: move |event| query.set(event.value()),
                    }
                }
                div { class: "flex flex-col gap-3",
                    p { class: "border-b border-black/[0.065] pb-2 text-[12px] font-medium uppercase tracking-[-0.01em] text-[rgba(15,15,16,0.5)]", "Categories" }
                    div { class: "flex flex-wrap gap-1.5",
                        for category in ArticleCategory::ALL {
                            {
                                let is_selected = current_selected.contains(&category);
                                let count = articles.iter().filter(|article| article.category == category).count();
                                rsx! {
                                    button {
                                        key: "{category.slug()}",
                                        type: "button",
                                        aria_pressed: is_selected,
                                        class: listing_chip_class(is_selected),
                                        onclick: move |_| {
                                            let mut next = selected();
                                            if let Some(index) = next.iter().position(|item| *item == category) {
                                                next.remove(index);
                                            } else {
                                                next.push(category);
                                            }
                                            selected.set(next);
                                        },
                                        "{category.label()}"
                                        span { class: "ml-1 tabular-nums opacity-50", "{count}" }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center justify-between",
                    button {
                        type: "button",
                        disabled: !is_filtering,
                        class: "text-[13px] font-medium text-[rgba(15,15,16,0.73)] underline-offset-2 hover:underline disabled:opacity-40 disabled:no-underline",
                        onclick: move |_| {
                            query.set(String::new());
                            selected.set(Vec::new());
                        },
                        "Reset filters"
                    }
                    span { class: "text-[13px] tabular-nums text-[rgba(15,15,16,0.5)]", "{filtered.len()} of {articles.len()}" }
                }
            }
            div { class: "flex flex-col gap-8",
                if filtered.is_empty() {
                    p { class: "py-16 text-center text-[15px] text-[rgba(15,15,16,0.5)]", "No articles match that filter." }
                } else {
                    div { class: "grid gap-3 sm:grid-cols-2 xl:grid-cols-3",
                        for article in filtered {
                            ArticleCard { key: "{article.slug}", article }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleThumbnail(
    article: Article,
    #[props(default)] feature: bool,
    #[props(default)] compact: bool,
    #[props(default)] hero: bool,
) -> Element {
    let size_class = if feature { "aspect-[1200/630]" } else { "aspect-[16/10]" };
    let text_class = if compact {
        "p-6 text-[12px] leading-[1.3]"
    } else if feature {
        "p-10 text-[18px] leading-[1.3]"
    } else {
        "p-3 text-[8px] leading-[1.3]"
    };
    let hero_class = if hero { "md:w-[420px] md:shrink-0" } else { "" };
    rsx! {
        div { class: "relative {size_class} {hero_class} w-full overflow-hidden rounded-[12px] bg-black/[0.03]",
            div {
                class: "absolute inset-0",
                style: "background: radial-gradient(circle at 0% 0%, rgba(245, 102, 0, 0.82), transparent 40%), radial-gradient(circle at 100% 100%, rgba(255, 173, 85, 0.62), transparent 44%), linear-gradient(135deg, #160903 0%, #090909 48%, #120803 100%);",
            }
            img {
                src: "/articles/article-thumbnail-noise.svg",
                alt: "",
                aria_hidden: "true",
                class: "pointer-events-none absolute inset-0 size-full object-cover",
                style: "opacity: 0.16; mix-blend-mode: screen;",
            }
            div {
                class: "absolute inset-0",
                style: "background: radial-gradient(circle at 50% 50%, transparent 30%, rgba(0, 0, 0, 0.28) 100%);",
            }
            div { class: "absolute inset-0 flex items-center justify-center text-center",
                span {
                    class: "{text_class} font-semibold tracking-[-0.01em] text-white",
                    style: "text-shadow: 0 1px 8px rgba(0, 0, 0, 0.35);",
                    "{article.short_title()}"
                }
            }
        }
    }
}

#[component]
fn CategoryCarousel() -> Element {
    let articles = all_articles();
    let rail = use_horizontal_rail("article-category");
    let carousel_script = rail.script();

    rsx! {
        section { class: "mt-28",
            div { class: "mb-8 flex items-end justify-between gap-4",
                div { class: "flex flex-col gap-3",
                    p { class: "font-mono text-[14px] uppercase tracking-[-0.01em] text-[rgba(15,15,16,0.73)]",
                        span { class: "text-primary", "[" }
                        " All categories "
                        span { class: "text-primary", "]" }
                    }
                    h2 { class: "text-[clamp(34px,5vw,52px)] font-semibold leading-[1.05] tracking-[-0.042em] text-[#0f0f10]", "Browse Categories" }
                }
                div { class: "hidden gap-2 sm:flex",
                    button {
                        id: rail.previous_id(),
                        type: "button",
                        aria_label: "Previous category",
                        class: "flex size-10 items-center justify-center rounded-full border border-black/[0.12] bg-white text-[#0f0f10] transition-[opacity,background-color] duration-300 hover:bg-black/[0.03] disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:bg-white",
                        ArrowLeft { class: "size-4" }
                    }
                    button {
                        id: rail.next_id(),
                        type: "button",
                        aria_label: "Next category",
                        class: "flex size-10 items-center justify-center rounded-full border border-black/[0.12] bg-white text-[#0f0f10] transition-[opacity,background-color] duration-300 hover:bg-black/[0.03] disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:bg-white",
                        ArrowRight { class: "size-4" }
                    }
                }
            }
            div { class: "relative left-1/2 w-screen -translate-x-1/2",
                div {
                    id: rail.rail_id(),
                    class: "flex snap-x snap-proximity gap-4 overflow-x-auto overscroll-x-contain scroll-pl-[max(1.5rem,calc((100vw-1200px)/2+1.5rem))] px-[max(1.5rem,calc((100vw-1200px)/2+1.5rem))] pb-4 [-webkit-overflow-scrolling:touch] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
                    for category in ArticleCategory::ALL {
                        {
                            let count = articles.iter().filter(|article| article.category == category).count();
                            rsx! {
                                Link {
                                    key: "{category.slug()}",
                                    to: Route::ArticleCategoryPage { category: category.slug().to_string() },
                                    class: "flex w-[300px] shrink-0 snap-start flex-col gap-5 rounded-[36px] border border-black/[0.05] bg-white p-6 shadow-md transition-transform duration-200 hover:-translate-y-0.5",
                                    div { class: "flex size-11 items-center justify-center rounded-[14px] border border-primary/30 bg-primary/10 text-primary",
                                        CategoryGlyph { category }
                                    }
                                    div { class: "flex flex-col gap-2",
                                        h3 { class: "text-[20px] font-semibold tracking-[-0.03em] text-[#0f0f10]", "{category.label()}" span { class: "ml-1.5 text-[14px] font-normal tabular-nums text-[rgba(15,15,16,0.4)]", "{count}" } }
                                        p { class: "text-[15px] leading-[1.45] text-[rgba(15,15,16,0.73)]", "{category.blurb()}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            script { "{carousel_script}" }
        }
    }
}

#[component]
fn CategoryGlyph(category: ArticleCategory) -> Element {
    match category {
        ArticleCategory::Foundations => rsx! { BookOpen { class: "size-5" } },
        ArticleCategory::Components => rsx! { Boxes { class: "size-5" } },
        ArticleCategory::Interaction => rsx! { Cog { class: "size-5" } },
        ArticleCategory::Production => rsx! { Rocket { class: "size-5" } },
    }
}

#[component]
fn ArticleClosingCta() -> Element {
    rsx! {
        section { class: "mt-20 rounded-[28px] border bg-muted/30 p-8 text-center md:p-12",
            p { class: "text-xs font-medium uppercase tracking-[0.16em] text-primary", "Keep building" }
            h2 { class: "mt-3 text-2xl font-semibold tracking-tight md:text-3xl", "Build a real Rust UI app" }
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

const fn listing_chip_class(selected: bool) -> &'static str {
    if selected {
        "rounded-full border border-primary/30 bg-primary/10 px-3 py-1 text-[12px] font-medium uppercase tracking-[-0.01em] text-primary transition-colors"
    } else {
        "rounded-full border border-black/[0.08] bg-black/[0.03] px-3 py-1 text-[12px] font-medium uppercase tracking-[-0.01em] text-[rgba(15,15,16,0.73)] transition-colors hover:bg-black/[0.06]"
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
