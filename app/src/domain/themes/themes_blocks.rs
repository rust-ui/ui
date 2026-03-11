use app_domain::themes::cards::chat::CardChat;
use app_domain::themes::cards::cookie_settings::CardCookieSettings;
use app_domain::themes::cards::create_account::CardCreateAccount;
use app_domain::themes::cards::exercise_minutes::CardExerciseMinutes;
use app_domain::themes::cards::move_goal::CardMoveGoal;
use app_domain::themes::cards::payment_method::CardPaymentMethod;
use app_domain::themes::cards::payments_table::CardPaymentsTable;
use app_domain::themes::cards::report_issue::CardReportIssue;
use app_domain::themes::cards::share_this_document::CardShareThisDocument;
use app_domain::themes::cards::subscriptions::CardSubscriptions;
use app_domain::themes::cards::team_members::CardTeamMembers;
use app_domain::themes::cards::total_revenue::CardTotalRevenue;
use leptos::prelude::*;
use registry::demos::demo_date_picker::DemoDatePicker;

#[component]
pub fn ThemesBlocks() -> impl IntoView {
    // Load chart scripts for the exercise minutes card (client-only, idempotent).
    Effect::new(move |_| {
        let Some(document) = window().document() else { return };
        let Some(head) = document.head() else { return };
        if document.get_element_by_id("chart-init-script").is_some() {
            return;
        }
        let Ok(script) = document.create_element("script") else { return };
        let _ = script.set_attribute("id", "chart-init-script");
        let _ = script.set_attribute("src", "/coming_soon/chart_init.js?v=6");
        let _ = head.append_child(&script);
    });

    view! {
        <div class="grid mx-2 md:gap-4 lg:grid-cols-10 xl:grid-cols-11 xl:gap-4 md:grids-col-2">
            <LeftSide />
            <RightSide />
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn LeftSide() -> impl IntoView {
    view! {
        <div class="space-y-4 lg:col-span-4 xl:col-span-6 xl:space-y-4">
            <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-2">
                <CardTotalRevenue />
                <CardSubscriptions />
            </div>

            <div class="grid gap-1 md:hidden sm:grid-cols-[280px_1fr]">
                <DemoDatePicker />
                // * Appears twice
                <CardMoveGoal />
                // * Appears twice
                <CardExerciseMinutes />
            </div>

            <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-1 xl:grid-cols-2">
                <div class="space-y-4 xl:space-y-4">
                    <CardTeamMembers />
                    <CardCookieSettings />
                    <CardPaymentMethod />
                </div>

                <div class="space-y-4 xl:space-y-4">
                    <CardChat />
                    <CardCreateAccount />
                    <div class="hidden xl:block">
                        <CardReportIssue />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RightSide() -> impl IntoView {
    view! {
        <div class="space-y-4 lg:col-span-6 xl:col-span-5 xl:space-y-4">
            <div class="hidden gap-1 md:grid sm:grid-cols-[280px_1fr]">
                <DemoDatePicker />
                <CardMoveGoal />
                <CardExerciseMinutes />
            </div>

            <div class="hidden md:block">
                <CardPaymentsTable />
            </div>

            <CardShareThisDocument />

            <div class="xl:hidden">
                <CardReportIssue />
            </div>
        </div>
    }
}
