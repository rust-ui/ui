use icons::Terminal;
use leptos::prelude::*;

use crate::ui::alert::{Alert, AlertDescription, AlertTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAlertRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-lg">
            <Alert>
                <Terminal />
                <AlertTitle>"تنبيه!"</AlertTitle>
                <AlertDescription>
                    "يمكنك إضافة المكونات إلى تطبيقك باستخدام واجهة سطر الأوامر."
                </AlertDescription>
            </Alert>
        </DirectionProvider>
    }
}
