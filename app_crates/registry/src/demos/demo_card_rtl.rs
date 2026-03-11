use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoCardRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-lg">
            <Card class="max-w-lg">
                <CardHeader>
                    <CardTitle>"عنوان البطاقة"</CardTitle>
                </CardHeader>

                <CardContent>
                    <CardDescription>
                        "مرحباً، هذا وصف تفصيلي لمحتوى البطاقة. يمكنك إضافة المزيد من النصوص هنا لتوفير معلومات إضافية حول غرض البطاقة وميزاتها وأي تفاصيل أخرى ذات صلة."
                    </CardDescription>
                </CardContent>

                <CardFooter class="justify-end">
                    <Button variant=ButtonVariant::Outline>"إلغاء"</Button>
                    <Button>"تأكيد"</Button>
                </CardFooter>
            </Card>
        </DirectionProvider>
    }
}
