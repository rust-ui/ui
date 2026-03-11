use app_components::LogoHomeLink;
use app_config::SiteConfig;
use app_routes::{BlockRoutes, ComponentsRoutes, HooksRoutes};
use icons::ExternalLink;
use leptos::prelude::*;
use leptos_ui::{clx, void};
use registry::ui::image::Image;

mod components {
    use super::*;
    clx! {StickyFooterWrapper, div, "w-full"}
    clx! {StickyFooterPanel, div, ""}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn StickyFooter() -> impl IntoView {
    clx! {FooterLink, a, "block mt-2 text-sm text-muted-foreground"}
    clx! {FooterTitle, h3, "text-base font-semibold uppercase"}
    void! {MySeparator, div, "shrink-0 bg-border h-[1px] w-full mt-10 mb-4"}

    view! {
        <div class="relative mt-2 h-[250px]" style="clip-path: polygon(0% 0px, 100% 0%, 100% 100%, 0px 100%);">
            <footer data-name="__StickyFooter" class="relative -top-[100vh] h-[calc(100vh+250px)]">
                <div class="sticky h-[250px] top-[calc(100vh-250px)]">
                    <div class="py-4 px-6 w-full h-full bg-accent">
                        <div class="flex justify-between w-full">
                            <div class="flex flex-col gap-4">
                                <LogoHomeLink />
                                <p class="max-w-xs text-sm text-muted-foreground">{SiteConfig::DESCRIPTION}</p>
                                <SocialLinks />
                            </div>

                            <div class="flex gap-8 items-start">
                                // TODO 🚑 SHORTFIX. Find a better fix for Mobile size
                                <div class="hidden flex-col md:flex">
                                    <FooterTitle>Components</FooterTitle>
                                    <FooterLink attr:href=ComponentsRoutes::Button.to_route()>Button</FooterLink>
                                    <FooterLink attr:href=ComponentsRoutes::AlertDialog
                                        .to_route()>Alert Dialog</FooterLink>
                                    <FooterLink attr:href=ComponentsRoutes::Breadcrumb
                                        .to_route()>Breadcrumb</FooterLink>
                                    <FooterLink attr:href=ComponentsRoutes::Sonner.to_route()>Sonner</FooterLink>
                                </div>
                                // TODO 🚑 SHORTFIX. Find a better fix for Mobile size
                                <div class="hidden flex-col md:flex">
                                    <FooterTitle>Blocks</FooterTitle>
                                    <FooterLink attr:href=BlockRoutes::Login.to_route()>Login</FooterLink>
                                    <FooterLink attr:href=BlockRoutes::Sidenav.to_route()>Sidenav</FooterLink>
                                    <FooterLink attr:href=BlockRoutes::Footers.to_route()>Footers</FooterLink>
                                </div>
                                // TODO 🚑 SHORTFIX. Find a better fix for Mobile size
                                <div class="hidden flex-col lg:flex">
                                    <FooterTitle>Hooks</FooterTitle>
                                    <FooterLink attr:href=HooksRoutes::UseCopyClipboard
                                        .to_route()>Use Copy Clipboard</FooterLink>
                                    <FooterLink attr:href=HooksRoutes::UseLockBodyScroll
                                        .to_route()>Use Lock Body Scroll</FooterLink>
                                    <FooterLink attr:href=HooksRoutes::UseRandom.to_route()>Use Random</FooterLink>
                                </div>

                            </div>
                        </div>

                        <MySeparator attr:data-orientation="horizontal" attr:role="none" />

                        <div class="flex justify-between items-center">
                            <p class="text-sm text-muted-foreground">
                                <span>"Built by "</span>
                                <a
                                    class="underline"
                                    href="https://www.rustify.rs"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                >
                                    "Rustify"
                                </a>
                                <span>"."</span>
                            </p>
                            <div class="flex relative gap-2 items-center">
                                <style>
                                    "@keyframes FerrisMove {
                                        0%, 100% { transform: translateX(0); }
                                        50% { transform: translateX(-200px); }
                                    }
                                    @keyframes FerrisWiggle {
                                        0%, 100% { transform: rotate(-3deg); }
                                        50% { transform: rotate(3deg); }
                                    }
                                    .animate-FerrisMove {
                                        animation: FerrisMove 8s infinite linear;
                                    }
                                    .animate-FerrisWiggle {
                                        animation: FerrisWiggle 0.25s infinite;
                                    }"
                                </style>

                                <div class="absolute top-1 -left-12 animate-FerrisMove">
                                    <Image
                                        src="/images/ferris_happy.png"
                                        alt="Ferris the Rust mascot"
                                        width=36
                                        height=36
                                        class="w-9 animate-FerrisWiggle"
                                    />
                                </div>
                                <a
                                    href="https://www.rustify.rs"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="flex gap-1 items-center py-1 px-2 text-sm font-medium rounded-md transition-colors bg-neutral-300 text-muted-foreground dark:bg-neutral-700 dark:hover:bg-neutral-600 hover:bg-neutral-400"
                                    title="Visit Rustify.rs"
                                >
                                    <span>"Rustify"</span>
                                    <ExternalLink class="size-4 text-muted-foreground" />
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn SocialLinks() -> impl IntoView {
    view! {
        <div class="flex gap-2 items-center text-muted-foreground">
            <a
                attr:href="https://github.com/rust-ui/labs"
                attr:target="_blank"
                attr:rel="noopener noreferrer"
                aria-label="GitHub"
                class="transition-colors dark:hover:text-neutral-300 hover:text-neutral-600"
            >
                <svg
                    viewBox="0 0 438.549 438.549"
                    width="24"
                    height="24"
                    fill="currentColor"
                    xmlns="http://www.w3.org/2000/svg"
                    class="size-4"
                >
                    <title>GitHub</title>
                    <path d="M409.132 114.573c-19.608-33.596-46.205-60.194-79.798-79.8-33.598-19.607-70.277-29.408-110.063-29.408-39.781 0-76.472 9.804-110.063 29.408-33.596 19.605-60.192 46.204-79.8 79.8C9.803 148.168 0 184.854 0 224.63c0 47.78 13.94 90.745 41.827 128.906 27.884 38.164 63.906 64.572 108.063 79.227 5.14.954 8.945.283 11.419-1.996 2.475-2.282 3.711-5.14 3.711-8.562 0-.571-.049-5.708-.144-15.417a2549.81 2549.81 0 01-.144-25.406l-6.567 1.136c-4.187.767-9.469 1.092-15.846 1-6.374-.089-12.991-.757-19.842-1.999-6.854-1.231-13.229-4.086-19.13-8.559-5.898-4.473-10.085-10.328-12.56-17.556l-2.855-6.57c-1.903-4.374-4.899-9.233-8.992-14.559-4.093-5.331-8.232-8.945-12.419-10.848l-1.999-1.431c-1.332-.951-2.568-2.098-3.711-3.429-1.142-1.331-1.997-2.663-2.568-3.997-.572-1.335-.098-2.43 1.427-3.289 1.525-.859 4.281-1.276 8.28-1.276l5.708.853c3.807.763 8.516 3.042 14.133 6.851 5.614 3.806 10.229 8.754 13.846 14.842 4.38 7.806 9.657 13.754 15.846 17.847 6.184 4.093 12.419 6.136 18.699 6.136 6.28 0 11.704-.476 16.274-1.423 4.565-.952 8.848-2.383 12.847-4.285 1.713-12.758 6.377-22.559 13.988-29.41-10.848-1.14-20.601-2.857-29.264-5.14-8.658-2.286-17.605-5.996-26.835-11.14-9.235-5.137-16.896-11.516-22.985-19.126-6.09-7.614-11.088-17.61-14.987-29.979-3.901-12.374-5.852-26.648-5.852-42.826 0-23.035 7.52-42.637 22.557-58.817-7.044-17.318-6.379-36.732 1.997-58.24 5.52-1.715 13.706-.428 24.554 3.853 10.85 4.283 18.794 7.952 23.84 10.994 5.046 3.041 9.089 5.618 12.135 7.708 17.705-4.947 35.976-7.421 54.818-7.421s37.117 2.474 54.823 7.421l10.849-6.849c7.419-4.57 16.18-8.758 26.262-12.565 10.088-3.805 17.802-4.853 23.134-3.138 8.562 21.509 9.325 40.922 2.279 58.24 15.036 16.18 22.559 35.787 22.559 58.817 0 16.178-1.958 30.497-5.853 42.966-3.9 12.471-8.941 22.457-15.125 29.979-6.191 7.521-13.901 13.85-23.131 18.986-9.232 5.14-18.182 8.85-26.84 11.136-8.662 2.286-18.415 4.004-29.263 5.146 9.894 8.562 14.842 22.077 14.842 40.539v60.237c0 3.422 1.19 6.279 3.572 8.562 2.379 2.279 6.136 2.95 11.276 1.995 44.163-14.653 80.185-41.062 108.068-79.226 27.88-38.161 41.825-81.126 41.825-128.906-.01-39.771-9.818-76.454-29.414-110.049z" />
                </svg>
            </a>
            <a
                attr:href="https://www.linkedin.com/company/rust-ui/"
                attr:target="_blank"
                attr:rel="noopener noreferrer"
                aria-label="LinkedIn"
                class="transition-colors dark:hover:text-neutral-300 hover:text-neutral-600"
            >
                <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    xmlns="http://www.w3.org/2000/svg"
                    class="size-4"
                >
                    <title>LinkedIn</title>
                    <path d="M19 0h-14c-2.76 0-5 2.24-5 5v14c0 2.76 2.24 5 5 5h14c2.76 0 5-2.24 5-5v-14c0-2.76-2.24-5-5-5zm-11 19h-3v-9h3v9zm-1.5-10.28c-.97 0-1.75-.79-1.75-1.75s.78-1.75 1.75-1.75 1.75.79 1.75 1.75-.78 1.75-1.75 1.75zm15.5 10.28h-3v-4.5c0-1.1-.9-2-2-2s-2 .9-2 2v4.5h-3v-9h3v1.22c.41-.72 1.39-1.22 2.25-1.22 1.93 0 3.5 1.57 3.5 3.5v5.5z" />
                </svg>
            </a>
            <a
                attr:href="https://discord.gg/mbszS27TqA"
                attr:target="_blank"
                attr:rel="noopener noreferrer"
                aria-label="Discord"
                class="transition-colors dark:hover:text-neutral-300 hover:text-neutral-600"
            >
                <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    xmlns="http://www.w3.org/2000/svg"
                    class="size-4"
                >
                    <title>Discord</title>
                    <path d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.6083 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1636-.3933-.4058-.8742-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276c-.598.3428-1.2205.6447-1.8733.8923a.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1835 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9555-2.4189 2.157-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.9555 2.4189-2.1569 2.4189zm7.9748 0c-1.1836 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.4189-2.1568 2.4189z" />
                </svg>
            </a>
        </div>
    }
}
