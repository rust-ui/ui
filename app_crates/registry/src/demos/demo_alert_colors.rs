use icons::TriangleAlert;
use leptos::prelude::*;

use crate::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
pub fn DemoAlertColors() -> impl IntoView {
    view! {
        <Alert class="max-w-md text-amber-900 bg-amber-50 border-amber-200 dark:text-amber-50 dark:border-amber-900 dark:bg-amber-950">
            <TriangleAlert />
            <AlertTitle>"Your subscription will expire in 3 days."</AlertTitle>
            <AlertDescription>
                "Renew now to avoid service interruption or upgrade to a paid plan to continue using the service."
            </AlertDescription>
        </Alert>
    }
}
