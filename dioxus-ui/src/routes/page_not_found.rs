use dioxus::prelude::*;
use icons::{Anchor, Component, House};
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

use crate::components::navigation::header_docs::HeaderDocs;

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    let _ = segments;

    // Report the 404 server-side (fire-and-forget). URL comes from the request
    // parts held by the current fullstack context during streaming SSR.
    #[cfg(feature = "server")]
    {
        use dioxus::fullstack::FullstackContext;
        let url = FullstackContext::current().map(|ctx| ctx.parts_mut().uri.to_string());
        crate::domain::bug_report::bug_reports::report_not_found(url);
    }

    rsx! {
        document::Title { "Rust/UI · 404 Not Found" }

        HeaderDocs {}

        div { class: "flex flex-col justify-center items-center px-4 mt-10",
            div { class: "flex flex-col items-center space-y-8 max-w-3xl text-center",
                h1 { class: "text-9xl font-bold tracking-tight", "404" }

                h2 { class: "text-3xl font-bold", "Oops! Page Not Found" }

                p { class: "max-w-2xl text-lg text-muted-foreground",
                    "Looks like this page took a creative detour! Don't worry, even the best designs sometimes need a redirect. Let's get you back on track to discover amazing web experiences."
                }

                div { class: "flex gap-4",
                    a { href: "/",
                        Button { size: ButtonSize::Lg,
                            House { class: "mr-2" }
                            "Return Home"
                        }
                    }
                    a { href: "/docs/components/button",
                        Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Browse Components" }
                    }
                }

                // Explore Section
                div { class: "pt-8 w-full",
                    h3 { class: "mb-6 text-xl font-semibold", "Explore Our Registry" }
                    div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3",
                        a { href: "/docs/components/accordion",
                            Card { class: "transition-all hover:shadow-lg hover:scale-[1.02]",
                                CardHeader {
                                    div { class: "flex gap-3 items-center",
                                        div { class: "flex justify-center items-center w-10 h-10 rounded-lg bg-primary/10",
                                            Component { class: "w-5 h-5 text-primary" }
                                        }
                                        CardTitle { "Components" }
                                    }
                                }
                                CardContent {
                                    CardDescription { "Discover our collection of reusable UI components." }
                                }
                            }
                        }

                        a { href: "/docs/hooks/use-copy-clipboard",
                            Card { class: "transition-all hover:shadow-lg hover:scale-[1.02]",
                                CardHeader {
                                    div { class: "flex gap-3 items-center",
                                        div { class: "flex justify-center items-center w-10 h-10 rounded-lg bg-primary/10",
                                            Anchor { class: "w-5 h-5 text-primary" }
                                        }
                                        CardTitle { "Hooks" }
                                    }
                                }
                                CardContent {
                                    CardDescription { "Learn about powerful hooks for enhanced functionality." }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
