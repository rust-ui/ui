use app_domain::markdown_config::RegistryEntry;
use icons::ExternalLink;
use leptos::prelude::*;
use leptos_ui::clx;
use markdown_crate::MdFile;
use registry::ui::button::Button;

#[derive(Debug, Clone)]
pub struct TocItem {
    pub title: String,
    pub level: u8,
    pub anchor: String,
}

fn create_anchor_id(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn extract_toc_from_md(md_content: &MdFile<RegistryEntry>) -> Vec<TocItem> {
    let mut toc_items = Vec::new();

    for line in md_content.content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("##") && !trimmed.starts_with("###") {
            // H2 heading
            if let Some(title_str) = trimmed.strip_prefix("##") {
                let title = title_str.trim().to_string();
                let anchor = create_anchor_id(&title);
                toc_items.push(TocItem { title, level: 2, anchor });
            }
        } else if trimmed.starts_with("###") && !trimmed.starts_with("####") {
            // H3 heading
            if let Some(title_str) = trimmed.strip_prefix("###") {
                let title = title_str.trim().to_string();
                let anchor = create_anchor_id(&title);
                toc_items.push(TocItem { title, level: 3, anchor });
            }
        } else if trimmed.starts_with("####") && !trimmed.starts_with("#####") {
            // H4 heading
            if let Some(title_str) = trimmed.strip_prefix("####") {
                let title = title_str.trim().to_string();
                let anchor = create_anchor_id(&title);
                toc_items.push(TocItem { title, level: 4, anchor });
            }
        } else if trimmed.starts_with("#####") && !trimmed.starts_with("######") {
            // H5 heading
            if let Some(title_str) = trimmed.strip_prefix("#####") {
                let title = title_str.trim().to_string();
                let anchor = create_anchor_id(&title);
                toc_items.push(TocItem { title, level: 5, anchor });
            }
        } else if trimmed.starts_with("######") {
            // H6 heading
            if let Some(title_str) = trimmed.strip_prefix("######") {
                let title = title_str.trim().to_string();
                let anchor = create_anchor_id(&title);
                toc_items.push(TocItem { title, level: 6, anchor });
            }
        }
    }

    toc_items
}

#[component]
pub fn TableOfContents(toc_items: Vec<TocItem>) -> impl IntoView {
    clx! {TocLink, a, "block text-sm text-muted-foreground hover:text-foreground aria-[current=true]:text-foreground no-underline transition-colors data-[depth=3]:pl-4 data-[depth=4]:pl-6"}

    view! {
        <aside
            data-name="TableOfContents"
            class="hidden sticky lg:block top-18 h-[calc(100vh-3.5rem)] w-[250px] shrink-0"
        >
            <div class="flex flex-col pl-4 h-full">
                <div>
                    <h4 class="mb-4 text-sm font-medium">"On This Page"</h4>

                    <nav
                        data-name="TableOfContents"
                        aria-label="Table of contents"
                        class="overflow-y-auto max-h-[calc(100vh-28rem)] scrollbar__on_hover scroll-smooth"
                    >
                        <ul data-name="TocList" class="pb-4 space-y-1.5">
                            {toc_items
                                .into_iter()
                                .map(|item| {
                                    view! {
                                        <li>
                                            <TocLink attr:href=format!("#{}", item.anchor) attr:data-depth=item.level>
                                                {item.title}
                                            </TocLink>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </ul>
                    </nav>
                </div>

                <TocCTACard />
            </div>
        </aside>

        <script src="/app/table_of_contents.js" />
    }
    .into_any()
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn TocCTACard() -> impl IntoView {
    view! {
        <a
            href="https://rustify.rs/bootcamps/fullstack?utm_source=rust-ui&utm_medium=sidebar&utm_campaign=bootcamp-cta"
            target="_blank"
            rel="noopener noreferrer"
            class="flex flex-col gap-2 p-6 mt-6 text-sm no-underline rounded-xl transition-colors group bg-muted hover:bg-muted/80"
        >
            <h4 class="text-base font-semibold leading-tight group-hover:underline text-balance">
                "Learn Fullstack Rust with Rustify.rs"
            </h4>
            <span class="text-muted-foreground">"Trusted by many developers worldwide."</span>
            <span class="text-muted-foreground">
                "Rustify.rs provides a 9-Week Bootcamp to learn how to build Fullstack cross-platform apps."
            </span>

            <Button class="mt-2 pointer-events-auto group/btn">
                <span>"Learn more"</span>
                <ExternalLink class="transition-transform group-hover/btn:translate-x-0.5 group-hover/btn:-translate-y-0.5" />
            </Button>
        </a>
    }
}
