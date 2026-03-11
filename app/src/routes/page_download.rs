use app_config::{SeoMeta, SiteConfig};
use icons::Download;
use leptos::prelude::*;
use registry::blocks::footer01::Footer01;
use registry::ui::badge::{Badge, BadgeVariant};
use registry::ui::button::{Button, ButtonSize};
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use strum::{EnumIter, IntoEnumIterator};

use crate::components::navigation::header_docs::HeaderDocs;

const BASE_DOWNLOAD_URL: &str = "https://github.com/rust-ui/releases/releases/latest/download";

struct DownloadFile {
    label: &'static str,
    filename: &'static str,
    size: &'static str,
}

impl DownloadFile {
    fn url(&self) -> String {
        format!("{}/{}", BASE_DOWNLOAD_URL, self.filename)
    }
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum Platform {
    MacOs,
    Linux,
    Windows,
}

impl Platform {
    fn title(self) -> &'static str {
        match self {
            Platform::MacOs => "macOS",
            Platform::Linux => "Linux",
            Platform::Windows => "Windows",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            Platform::MacOs => "Apple Silicon (M1/M2/M3/M4)",
            Platform::Linux => "x86_64",
            Platform::Windows => "x64",
        }
    }

    fn downloads(self) -> &'static [DownloadFile] {
        match self {
            Platform::MacOs => &[DownloadFile { label: ".dmg", filename: "rust-ui.dmg", size: "1.6 MB" }],
            Platform::Linux => &[
                DownloadFile { label: ".AppImage", filename: "rust-ui_amd64.AppImage", size: "~5 MB" },
                DownloadFile { label: ".deb", filename: "rust-ui_amd64.deb", size: "~3 MB" },
            ],
            Platform::Windows => &[
                DownloadFile { label: ".msi", filename: "rust-ui_x64_en-US.msi", size: "~4 MB" },
                DownloadFile { label: ".exe", filename: "rust-ui_x64-setup.exe", size: "~4 MB" },
            ],
        }
    }

    fn requirements(self) -> &'static [&'static str] {
        match self {
            Platform::MacOs => &["macOS 11.0 (Big Sur) or later", "Apple Silicon (M1, M2, M3, M4)", "~4 MB disk space"],
            Platform::Linux => {
                &["Ubuntu 22.04 / Debian 11 or later", "x86_64 architecture", "WebKit2GTK 4.1", "~5 MB disk space"]
            }
            Platform::Windows => &["Windows 10 or later", "x64 architecture", "~4 MB disk space"],
        }
    }
}

#[component]
pub fn PageDownload() -> impl IntoView {
    let title = format!("Download Rust UI Desktop | {}", SiteConfig::TITLE);
    let description = "Download Rust UI Desktop app for macOS, Linux, and Windows. Browse and preview UI components offline with our native desktop application.".to_string();
    let canonical_url = format!("{}/download", SiteConfig::BASE_URL);

    view! {
        <SeoMeta title=title description=description canonical_url=canonical_url />

        <HeaderDocs />

        <div class="flex flex-col gap-12 items-center py-16 px-4 mx-auto max-w-4xl">
            <div class="flex flex-col gap-4 items-center text-center">
                <Badge variant=BadgeVariant::Secondary>"Desktop App"</Badge>
                <h1 class="text-4xl font-bold tracking-tight sm:text-5xl">"Rust UI Desktop"</h1>
                <p class="max-w-2xl text-lg text-muted-foreground">
                    "Browse and preview UI components offline with our native desktop application. Built with Tauri for a fast, lightweight experience."
                </p>
            </div>

            <div class="grid gap-6 w-full max-w-2xl">
                {Platform::iter()
                    .map(|platform| {
                        view! {
                            <Card class="transition-all hover:shadow-lg">
                                <CardHeader>
                                    <div class="flex gap-4 justify-between items-center">
                                        <div class="flex flex-col gap-1">
                                            <CardTitle>{platform.title()}</CardTitle>
                                            <CardDescription>{platform.subtitle()}</CardDescription>
                                        </div>
                                        <div class="flex gap-2">
                                            {platform
                                                .downloads()
                                                .iter()
                                                .map(|file| {
                                                    let url = file.url();
                                                    view! {
                                                        <Button
                                                            href=url
                                                            attr:download=file.filename
                                                            size=ButtonSize::Lg
                                                        >
                                                            <Download class="size-5" />
                                                            <span>{file.label}</span>
                                                        </Button>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    </div>
                                </CardHeader>
                                <CardContent>
                                    <div class="flex flex-wrap gap-3 text-sm text-muted-foreground">
                                        {platform
                                            .downloads()
                                            .iter()
                                            .map(|file| {
                                                view! { <span>{file.filename}" · "{file.size}</span> }
                                            })
                                            .collect_view()}
                                    </div>
                                </CardContent>
                            </Card>
                        }
                    })
                    .collect_view()}
            </div>

            <div class="grid gap-8 w-full max-w-2xl sm:grid-cols-3">
                {Platform::iter()
                    .map(|platform| {
                        view! {
                            <div class="flex flex-col gap-2">
                                <h2 class="text-sm font-semibold">{platform.title()}</h2>
                                <ul class="space-y-1 text-sm text-muted-foreground">
                                    {platform.requirements().iter().map(|req| view! { <li>{*req}</li> }).collect_view()}
                                </ul>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>

        <Footer01 />
    }
}
