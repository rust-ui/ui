use dioxus::prelude::*;
use registry::demos::demo_date_picker::DemoDatePicker;

use super::cards::chat::CardChat;
use super::cards::cookie_settings::CardCookieSettings;
use super::cards::create_account::CardCreateAccount;
use super::cards::exercise_minutes::CardExerciseMinutes;
use super::cards::move_goal::CardMoveGoal;
use super::cards::payment_method::CardPaymentMethod;
use super::cards::payments_table::CardPaymentsTable;
use super::cards::report_issue::CardReportIssue;
use super::cards::share_this_document::CardShareThisDocument;
use super::cards::subscriptions::CardSubscriptions;
use super::cards::team_members::CardTeamMembers;
use super::cards::total_revenue::CardTotalRevenue;

#[component]
pub fn ThemesBlocks() -> Element {
    use_effect(move || {
        spawn(async move {
            let js = r#"
                (function() {
                    if (document.getElementById('chart-init-script')) return;
                    var s = document.createElement('script');
                    s.id = 'chart-init-script';
                    s.src = '/app_components/chart_init.js?v=6';
                    document.head.appendChild(s);
                })();
            "#;
            dioxus::document::eval(js).await.ok();
        });
    });

    rsx! {
        div { class: "grid mx-2 md:gap-4 lg:grid-cols-10 xl:grid-cols-11 xl:gap-4 md:grids-col-2",
            LeftSide {}
            RightSide {}
        }
    }
}

#[component]
fn LeftSide() -> Element {
    rsx! {
        div { class: "space-y-4 lg:col-span-4 xl:col-span-6 xl:space-y-4",
            div { class: "grid gap-4 sm:grid-cols-2 xl:grid-cols-2",
                CardTotalRevenue {}
                CardSubscriptions {}
            }

            div { class: "grid gap-4 md:grid-cols-2 lg:grid-cols-1 xl:grid-cols-2",
                div { class: "space-y-4 xl:space-y-4",
                    CardTeamMembers {}
                    CardCookieSettings {}
                    CardPaymentMethod {}
                }
                div { class: "space-y-4 xl:space-y-4",
                    CardChat {}
                    CardCreateAccount {}
                    div { class: "hidden xl:block",
                        CardReportIssue {}
                    }
                }
            }
        }
    }
}

#[component]
fn RightSide() -> Element {
    rsx! {
        div { class: "space-y-4 lg:col-span-6 xl:col-span-5 xl:space-y-4",
            div { class: "hidden gap-1 md:grid sm:grid-cols-[280px_1fr]",
                DemoDatePicker {}
                CardMoveGoal {}
                CardExerciseMinutes {}
            }

            div { class: "hidden md:block",
                CardPaymentsTable {}
            }

            CardShareThisDocument {}

            div { class: "xl:hidden",
                CardReportIssue {}
            }
        }
    }
}
