use leptos::prelude::*;

use crate::ui::navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
    NavigationMenuTrigger, navigation_menu_trigger_style,
};

#[component]
fn ListItem(#[prop(into)] href: String, #[prop(into)] title: String, children: Children) -> impl IntoView {
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
pub fn DemoNavigationMenu() -> impl IntoView {
    view! {
        <div class="flex justify-center items-start py-8 min-h-[350px]">
            <NavigationMenu>
                <NavigationMenuList>
                    <NavigationMenuItem>
                        <NavigationMenuTrigger>"Getting Started"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul class="grid gap-3 p-0 md:grid-cols-2 w-[400px] md:w-[500px] lg:w-[600px]">
                                <li class="row-span-3">
                                    <a
                                        href="#"
                                        class="flex flex-col justify-end p-6 w-full h-full no-underline bg-gradient-to-b rounded-md outline-none select-none focus:shadow-md from-muted/50 to-muted hover:bg-accent"
                                    >
                                        <div class="mt-4 mb-2 text-lg font-medium">"rust/ui"</div>
                                        <p class="text-sm leading-tight text-muted-foreground">
                                            "Beautifully designed components built with Leptos and Tailwind CSS."
                                        </p>
                                    </a>
                                </li>
                                <ListItem href="#" title="Introduction">
                                    "Re-usable components built using Leptos and Tailwind CSS."
                                </ListItem>
                                <ListItem href="#" title="Installation">
                                    "How to install dependencies and structure your app."
                                </ListItem>
                                <ListItem href="#" title="Typography">
                                    "Styles for headings, paragraphs, lists and more."
                                </ListItem>
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuTrigger>"Components"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul class="grid gap-3 p-0 md:grid-cols-2 w-[400px] md:w-[500px] lg:w-[600px]">
                                <ListItem href="#" title="Alert">
                                    "Displays a callout for user attention."
                                </ListItem>
                                <ListItem href="#" title="Alert Dialog">
                                    "A modal dialog that interrupts the user."
                                </ListItem>
                                <ListItem href="#" title="Button">
                                    "Triggers an action or event."
                                </ListItem>
                                <ListItem href="#" title="Card">
                                    "Displays content in a card container."
                                </ListItem>
                                <ListItem href="#" title="Input">
                                    "Displays a form input field."
                                </ListItem>
                                <ListItem href="#" title="Select">
                                    "Displays a list of options to pick from."
                                </ListItem>
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuLink class=navigation_menu_trigger_style() href="#">
                            "Documentation"
                        </NavigationMenuLink>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
        </div>
    }
}
