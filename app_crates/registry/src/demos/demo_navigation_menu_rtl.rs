use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
    NavigationMenuTrigger, navigation_menu_trigger_style,
};

#[component]
fn ListItemRtl(#[prop(into)] href: String, #[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <li>
            <a
                href=href
                class="block p-3 space-y-1 leading-none no-underline rounded-md transition-colors outline-none select-none hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground"
            >
                <div class="text-sm font-medium leading-none">{title}</div>
                <p class="text-sm leading-snug line-clamp-2 text-muted-foreground">{children()}</p>
            </a>
        </li>
    }
}

#[component]
pub fn DemoNavigationMenuRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-2xl">
            <div class="flex justify-center items-start py-8 min-h-[350px]">
                <NavigationMenu>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                            <NavigationMenuTrigger>"البدء"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <ul class="grid gap-3 p-0 md:grid-cols-2 w-[400px] md:w-[500px] lg:w-[600px]">
                                    <li class="row-span-3">
                                        <a
                                            href="#"
                                            class="flex flex-col justify-end p-6 w-full h-full no-underline bg-gradient-to-b rounded-md outline-none select-none focus:shadow-md from-muted/50 to-muted hover:bg-accent"
                                        >
                                            <div class="mt-4 mb-2 text-lg font-medium">"rust/ui"</div>
                                            <p class="text-sm leading-tight text-muted-foreground">
                                                "مكونات مصممة بعناية باستخدام Leptos و Tailwind CSS."
                                            </p>
                                        </a>
                                    </li>
                                    <ListItemRtl href="#" title="مقدمة">
                                        "مكونات قابلة لإعادة الاستخدام مبنية باستخدام Leptos و Tailwind CSS."
                                    </ListItemRtl>
                                    <ListItemRtl href="#" title="التثبيت">
                                        "كيفية تثبيت الاعتماديات وهيكلة تطبيقك."
                                    </ListItemRtl>
                                    <ListItemRtl href="#" title="الطباعة">
                                        "أنماط للعناوين والفقرات والقوائم وغيرها."
                                    </ListItemRtl>
                                </ul>
                            </NavigationMenuContent>
                        </NavigationMenuItem>

                        <NavigationMenuItem>
                            <NavigationMenuTrigger>"المكونات"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <ul class="grid gap-3 p-0 md:grid-cols-2 w-[400px] md:w-[500px] lg:w-[600px]">
                                    <ListItemRtl href="#" title="تنبيه">
                                        "يعرض تنبيهاً لانتباه المستخدم."
                                    </ListItemRtl>
                                    <ListItemRtl href="#" title="حوار التنبيه">
                                        "حوار نمطي يقاطع المستخدم."
                                    </ListItemRtl>
                                    <ListItemRtl href="#" title="زر">
                                        "يُشغّل إجراءً أو حدثاً."
                                    </ListItemRtl>
                                    <ListItemRtl href="#" title="بطاقة">
                                        "يعرض المحتوى في حاوية بطاقة."
                                    </ListItemRtl>
                                </ul>
                            </NavigationMenuContent>
                        </NavigationMenuItem>

                        <NavigationMenuItem>
                            <NavigationMenuLink class=navigation_menu_trigger_style() href="#">
                                "التوثيق"
                            </NavigationMenuLink>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </div>
        </DirectionProvider>
    }
}
