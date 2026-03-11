use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::server_fn::codec::{GetUrl, Json, PostUrl};
use registry::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use registry::ui::button::ButtonVariant;
use registry::ui::dialog::DialogAction;

use crate::domain::bug_report::bug_reports_sqlite::StoredBugReport;

/// Parsed user agent information using static strings where possible.
/// All parsing methods are panic-free and handle malformed input gracefully.
#[derive(Debug, Clone, Default)]
struct ParsedUserAgent {
    browser: &'static str,
    browser_version: String,
    os: &'static str,
    os_version: String,
    device_type: &'static str,
    is_mobile: bool,
}

impl ParsedUserAgent {
    fn parse(ua: &str) -> Self {
        let ua_lower = ua.to_lowercase();
        let is_mobile = ua_lower.contains("mobile")
            || ua_lower.contains("android")
            || ua_lower.contains("iphone")
            || ua_lower.contains("ipad");
        let is_tablet = ua_lower.contains("ipad") || (ua_lower.contains("android") && !ua_lower.contains("mobile"));
        let device_type = if is_tablet {
            "Tablet"
        } else if is_mobile {
            "Mobile"
        } else {
            "Desktop"
        };
        let (os, os_version) = Self::detect_os(ua);
        let (browser, browser_version) = Self::detect_browser(ua);
        Self { browser, browser_version, os, os_version, device_type, is_mobile: is_mobile || is_tablet }
    }

    fn detect_os(ua: &str) -> (&'static str, String) {
        match () {
            _ if ua.contains("iPhone OS") || ua.contains("iPad") => ("iOS", Self::extract_version(ua, "OS ", " ")),
            _ if ua.contains("Android") => ("Android", Self::extract_version(ua, "Android ", ";")),
            _ if ua.contains("Mac OS X") => ("macOS", Self::extract_version(ua, "Mac OS X ", ")").replace('_', ".")),
            _ if ua.contains("Windows NT") => {
                let raw_version = Self::extract_version(ua, "Windows NT ", ";");
                let version = match raw_version.as_str() {
                    "10.0" => "10/11".to_string(),
                    "6.3" => "8.1".to_string(),
                    "6.1" => "7".to_string(),
                    _ => raw_version,
                };
                ("Windows", version)
            }
            _ if ua.contains("Linux") => ("Linux", String::new()),
            _ => ("Unknown", String::new()),
        }
    }

    fn detect_browser(ua: &str) -> (&'static str, String) {
        match () {
            _ if ua.contains("Edg/") => ("Edge", Self::extract_version(ua, "Edg/", " ")),
            _ if ua.contains("Chrome/") && !ua.contains("Chromium") => {
                ("Chrome", Self::extract_version(ua, "Chrome/", " "))
            }
            _ if ua.contains("Safari/") && !ua.contains("Chrome") => {
                ("Safari", Self::extract_version(ua, "Version/", " "))
            }
            _ if ua.contains("Firefox/") => ("Firefox", Self::extract_version(ua, "Firefox/", " ")),
            _ => ("Unknown", String::new()),
        }
    }

    fn extract_version(ua: &str, start: &str, end: &str) -> String {
        ua.find(start)
            .map(|i| {
                let after = &ua[i + start.len()..];
                let end_pos = after.find(end).unwrap_or(after.len());
                after[..end_pos].split('.').take(2).collect::<Vec<_>>().join(".")
            })
            .unwrap_or_default()
    }

    fn browser_display(&self) -> String {
        if self.browser_version.is_empty() {
            self.browser.to_string()
        } else {
            format!("{} {}", self.browser, self.browser_version)
        }
    }

    fn os_display(&self) -> String {
        if self.os_version.is_empty() { self.os.to_string() } else { format!("{} {}", self.os, self.os_version) }
    }
}

#[component]
pub fn PageBugReports() -> impl IntoView {
    let reports_resource = Resource::new(|| (), |_| fetch_bug_reports(100));
    let delete_action = ServerAction::<DeleteBugReport>::new();
    let delete_all_action = ServerAction::<DeleteAllBugReports>::new();

    // Refetch when delete actions complete
    Effect::new(move || {
        delete_action.version().get();
        delete_all_action.version().get();
        reports_resource.refetch();
    });

    view! {
        <div class="p-8 mx-auto max-w-6xl">
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-bold">"Bug Reports"</h1>
                <AlertDialog>
                    <AlertDialogTrigger attr:class="bg-red-600 hover:bg-red-700 text-white">
                        "Delete All"
                    </AlertDialogTrigger>
                    <Portal>
                        <AlertDialogContent class="max-w-md">
                            <AlertDialogBody>
                                <AlertDialogHeader>
                                    <AlertDialogTitle>"Delete all bug reports?"</AlertDialogTitle>
                                    <AlertDialogDescription>
                                        "This action cannot be undone. All bug reports will be permanently deleted."
                                    </AlertDialogDescription>
                                </AlertDialogHeader>
                                <AlertDialogFooter>
                                    <AlertDialogClose>"Cancel"</AlertDialogClose>
                                    <DialogAction
                                        variant=ButtonVariant::Destructive
                                        on:click=move |_| {
                                            delete_all_action.dispatch(DeleteAllBugReports {});
                                        }
                                    >
                                        "Delete All"
                                    </DialogAction>
                                </AlertDialogFooter>
                            </AlertDialogBody>
                        </AlertDialogContent>
                    </Portal>
                </AlertDialog>
            </div>

            <Suspense fallback=move || {
                view! { <p class="text-muted-foreground">"Loading bug reports..."</p> }
            }>
                {move || {
                    reports_resource
                        .get()
                        .map(|result| {
                            match result {
                                Ok(reports) => {
                                    if reports.is_empty() {
                                        view! { <p class="text-muted-foreground">"No bug reports found."</p> }
                                            .into_any()
                                    } else {
                                        view! {
                                            <div class="space-y-4">
                                                {reports
                                                    .into_iter()
                                                    .map(|report| {
                                                        let similarity_hash = report.similarity_hash;
                                                        view! {
                                                            <BugReportCard
                                                                report=report
                                                                on_delete=move || {
                                                                    delete_action.dispatch(DeleteBugReport { similarity_hash });
                                                                }
                                                            />
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        }
                                            .into_any()
                                    }
                                }
                                Err(err) => {
                                    view! { <p class="text-red-500">"Error loading reports: " {err.to_string()}</p> }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn BugReportCard<F>(report: StoredBugReport, on_delete: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let expanded_signal = RwSignal::new(false);

    let bug_type_class = match report.bug_type.as_str() {
        "Wasm" => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
        "Server" => "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200",
        "Database" => "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
        "LeptosWarning" => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
        "BrowserWarning" => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
        _ => "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200",
    };

    let bug_type = report.bug_type.clone();
    let created_at = report.created_at.clone();
    let message = report.message.clone();
    let url = report.url.clone();
    let exception_message = report.exception_message.clone();
    let stack_trace = report.stack_trace.clone();
    let user_agent = report.user_agent.clone();
    let user_login = report.user_login.clone();
    let application = report.application.clone();
    let id = report.id;
    let similarity_hash = report.similarity_hash;
    let similar_count = report.similar_count;

    // Parse user agent for display
    let parsed_ua = user_agent.as_ref().map(|ua| ParsedUserAgent::parse(ua));

    view! {
        <div class="p-4 rounded-lg border bg-card">
            <div class="flex gap-4 justify-between items-start">
                <div class="flex-1 min-w-0">
                    <div class="flex flex-wrap gap-2 items-center mb-2">
                        <span class=format!("px-2 py-1 text-xs font-medium rounded {bug_type_class}")>{bug_type}</span>
                        {(similar_count > 1)
                            .then(|| {
                                view! {
                                    <span class="py-1 px-2 text-xs font-medium text-orange-800 bg-orange-100 rounded dark:text-orange-200 dark:bg-orange-900">
                                        {format!("x{similar_count}")}
                                    </span>
                                }
                            })}
                        // Device/Browser/OS badges
                        {parsed_ua
                            .as_ref()
                            .map(|ua| {
                                let device_class = if ua.is_mobile {
                                    "bg-pink-100 text-pink-800 dark:bg-pink-900 dark:text-pink-200"
                                } else {
                                    "bg-cyan-100 text-cyan-800 dark:bg-cyan-900 dark:text-cyan-200"
                                };
                                view! {
                                    <span class=format!(
                                        "px-2 py-1 text-xs font-medium rounded {device_class}",
                                    )>{ua.device_type}</span>
                                    <span class="py-1 px-2 text-xs font-medium rounded bg-slate-100 text-slate-800 dark:bg-slate-800 dark:text-slate-200">
                                        {ua.browser_display()}
                                    </span>
                                    <span class="py-1 px-2 text-xs font-medium text-emerald-800 bg-emerald-100 rounded dark:text-emerald-200 dark:bg-emerald-900">
                                        {ua.os_display()}
                                    </span>
                                }
                            })}
                        <span class="text-xs text-muted-foreground">{created_at}</span>
                        <span class="font-mono text-xs text-muted-foreground">"#" {id}</span>
                    </div>
                    <p class="text-sm font-medium truncate">{message}</p>
                    {url
                        .map(|u| {
                            view! { <p class="text-xs text-muted-foreground truncate">{u}</p> }
                        })}
                </div>
                <div class="flex gap-2">
                    <button
                        class="py-1 px-3 text-xs font-medium rounded border hover:bg-muted"
                        on:click=move |_| expanded_signal.update(|v| *v = !*v)
                    >
                        {move || if expanded_signal.get() { "Collapse" } else { "Expand" }}
                    </button>
                    <button
                        class="py-1 px-3 text-xs font-medium text-red-600 rounded border border-red-200 dark:border-red-800 hover:bg-red-50 dark:hover:bg-red-950"
                        on:click=move |_| on_delete()
                    >
                        "Delete"
                    </button>
                </div>
            </div>

            <Show when=move || expanded_signal.get()>
                <div class="mt-4 space-y-3 text-sm">
                    {exception_message
                        .clone()
                        .map(|msg| {
                            view! {
                                <div>
                                    <p class="font-medium text-muted-foreground">"Exception:"</p>
                                    <pre class="overflow-x-auto p-2 mt-1 text-xs rounded bg-muted">{msg}</pre>
                                </div>
                            }
                        })}
                    {stack_trace
                        .clone()
                        .map(|trace| {
                            view! {
                                <div>
                                    <p class="font-medium text-muted-foreground">"Stack Trace:"</p>
                                    <pre class="overflow-x-auto overflow-y-auto p-2 mt-1 max-h-64 text-xs rounded bg-muted">
                                        {trace}
                                    </pre>
                                </div>
                            }
                        })} // Metadata grid
                    <div class="grid grid-cols-2 gap-4 pt-3 border-t sm:grid-cols-4 border-muted">
                        <div>
                            <p class="text-xs font-medium text-muted-foreground">"Application"</p>
                            <p class="text-xs">{application.clone()}</p>
                        </div>
                        {user_login
                            .clone()
                            .map(|login| {
                                view! {
                                    <div>
                                        <p class="text-xs font-medium text-muted-foreground">"User Login"</p>
                                        <p class="text-xs">{login}</p>
                                    </div>
                                }
                            })}
                        <div>
                            <p class="text-xs font-medium text-muted-foreground">"Similar Reports"</p>
                            <p class="text-xs">{similar_count}</p>
                        </div>
                        <div>
                            <p class="text-xs font-medium text-muted-foreground">"Hash"</p>
                            <p class="font-mono text-xs">{format!("{similarity_hash:x}")}</p>
                        </div>
                    </div>
                    {user_agent
                        .clone()
                        .map(|ua| {
                            view! {
                                <div class="pt-3 border-t border-muted">
                                    <p class="text-xs font-medium text-muted-foreground">"User Agent:"</p>
                                    <p class="text-xs break-all text-muted-foreground">{ua}</p>
                                </div>
                            }
                        })}
                </div>
            </Show>
        </div>
    }
}

/* ========================================================== */
/*                     SERVER FUNCTIONS                       */
/* ========================================================== */

#[server(FetchBugReports, "/api", input = GetUrl, output = Json)]
pub async fn fetch_bug_reports(limit: i64) -> Result<Vec<StoredBugReport>, ServerFnError> {
    crate::domain::bug_report::bug_reports_sqlite::fetch_all_bug_reports(limit)
        .map_err(|err| ServerFnError::new(format!("Failed to fetch bug reports: {err}")))
}

#[server(DeleteBugReport, "/api", input = PostUrl, output = Json)]
pub async fn delete_bug_report(similarity_hash: i64) -> Result<usize, ServerFnError> {
    crate::domain::bug_report::bug_reports_sqlite::delete_bug_reports_by_hash(similarity_hash)
        .map_err(|err| ServerFnError::new(format!("Failed to delete bug reports: {err}")))
}

#[server(DeleteAllBugReports, "/api", input = PostUrl, output = Json)]
pub async fn delete_all_bug_reports() -> Result<usize, ServerFnError> {
    crate::domain::bug_report::bug_reports_sqlite::delete_all_bug_reports()
        .map_err(|err| ServerFnError::new(format!("Failed to delete all bug reports: {err}")))
}
