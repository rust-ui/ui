use crate::markdown::{Frontmatter, parse_md};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArticleCategory {
    Foundations,
    Components,
    Interaction,
    Production,
}

impl ArticleCategory {
    pub const ALL: [Self; 4] = [Self::Foundations, Self::Components, Self::Interaction, Self::Production];

    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Foundations => "foundations",
            Self::Components => "components",
            Self::Interaction => "interaction",
            Self::Production => "production",
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Foundations => "Foundations",
            Self::Components => "Components",
            Self::Interaction => "Interaction",
            Self::Production => "Production",
        }
    }

    #[must_use]
    pub const fn blurb(self) -> &'static str {
        match self {
            Self::Foundations => "Start with solid Rust UI building blocks.",
            Self::Components => "Compose interfaces that stay easy to change.",
            Self::Interaction => "Make state, forms, and feedback feel native.",
            Self::Production => "Ship Rust UI across platforms with confidence.",
        }
    }

    #[must_use]
    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|category| category.slug() == slug)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Article {
    pub slug: &'static str,
    pub category: ArticleCategory,
    pub raw: &'static str,
    pub order: u8,
}

impl Article {
    #[must_use]
    pub fn frontmatter(self) -> Frontmatter {
        parse_md(self.raw).0
    }

    #[must_use]
    pub fn body(self) -> &'static str {
        parse_md(self.raw).1
    }

    #[must_use]
    pub fn title(self) -> String {
        self.frontmatter().title
    }

    #[must_use]
    pub fn description(self) -> String {
        self.frontmatter().description
    }

    #[must_use]
    pub fn short_title(self) -> String {
        let fm = self.frontmatter();
        if fm.short_title.is_empty() { self.title() } else { fm.short_title }
    }

    #[must_use]
    pub fn reading_time(self) -> usize {
        self.body().split_whitespace().count().div_ceil(220).max(1)
    }
}

pub const ARTICLES: &[Article] = &[
    Article {
        slug: "build-your-first-dioxus-ui-in-rust",
        category: ArticleCategory::Foundations,
        raw: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/public/articles/build-your-first-dioxus-ui-in-rust.md"
        )),
        order: 0,
    },
    Article {
        slug: "reusable-dioxus-components-typed-props",
        category: ArticleCategory::Components,
        raw: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/public/articles/reusable-dioxus-components-typed-props.md"
        )),
        order: 1,
    },
    Article {
        slug: "dioxus-forms-state-validation",
        category: ArticleCategory::Interaction,
        raw: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/public/articles/dioxus-forms-state-validation.md")),
        order: 2,
    },
    Article {
        slug: "responsive-rust-ui-with-dioxus-tailwind",
        category: ArticleCategory::Components,
        raw: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/public/articles/responsive-rust-ui-with-dioxus-tailwind.md"
        )),
        order: 3,
    },
    Article {
        slug: "async-data-loading-states-dioxus",
        category: ArticleCategory::Interaction,
        raw: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/public/articles/async-data-loading-states-dioxus.md")),
        order: 4,
    },
    Article {
        slug: "ship-cross-platform-rust-ui-dioxus",
        category: ArticleCategory::Production,
        raw: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/public/articles/ship-cross-platform-rust-ui-dioxus.md"
        )),
        order: 5,
    },
];

#[must_use]
pub const fn all_articles() -> &'static [Article] {
    ARTICLES
}

#[must_use]
pub fn article_by_slug(slug: &str) -> Option<Article> {
    ARTICLES.iter().copied().find(|article| article.slug == slug)
}
