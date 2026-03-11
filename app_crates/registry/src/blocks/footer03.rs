use leptos::prelude::*;

use super::footer_logos::{
    BrandFooter, LogoFacebook, LogoInstagram, LogoLinkedIn, LogoThreads, LogoTikTok, LogoTwitter,
};
use crate::ui::footer::{
    Footer, FooterBrand, FooterBrandLink, FooterContainer, FooterCopyright, FooterExternalLink, FooterGrid, FooterLink,
    FooterLinksSection, FooterSection, FooterSectionsGrid, FooterSocialContainer, FooterTitle,
};

/*
 * title: Footer Grid with Social Links
 * iframe_height: 474px
*/

#[component]
pub fn Footer03() -> impl IntoView {
    view! {
        <Footer class="pt-20">
            <FooterContainer>
                <FooterGrid>
                    <FooterBrand>
                        <FooterBrandLink attr:aria-label="go home" attr:href="/">
                            <BrandFooter />
                        </FooterBrandLink>
                    </FooterBrand>
                    <FooterSectionsGrid class="grid-cols-2 sm:grid-cols-4 md:col-span-3">
                        <FooterLinksSection>
                            <FooterTitle>Components</FooterTitle>
                            <FooterLink attr:href="/docs/components/accordion">Accordion</FooterLink>
                            <FooterLink attr:href="/docs/components/button">Button</FooterLink>
                            <FooterLink attr:href="/docs/components/card">Card</FooterLink>
                            <FooterLink attr:href="/docs/components/chips">Chips</FooterLink>
                            <FooterLink attr:href="/docs/components/dialog">Dialog</FooterLink>
                            <FooterLink attr:href="/docs/components/dropdown-menu">Dropdown</FooterLink>
                            <FooterLink attr:href="/docs/components/input">Input</FooterLink>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Hooks</FooterTitle>
                            <FooterLink attr:href="/docs/hooks/use-copy-clipboard">Use Copy Clipboard</FooterLink>
                            <FooterLink attr:href="/docs/hooks/use-lock-body-scroll">Use Lock Body Scroll</FooterLink>
                            <FooterLink attr:href="/docs/hooks/use-random">Use Random</FooterLink>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Blocks</FooterTitle>
                            <FooterLink attr:href="/blocks/login">Login</FooterLink>
                            <FooterLink attr:href="/blocks/sidenav">Sidenav</FooterLink>
                            <FooterLink attr:href="/blocks/headers">Headers</FooterLink>
                            <FooterLink attr:href="/blocks/footers">Footers</FooterLink>
                            <FooterLink attr:href="/blocks/faq">FAQ</FooterLink>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Legal</FooterTitle>
                            <FooterLink attr:href="#">Licence</FooterLink>
                            <FooterLink attr:href="#">Privacy</FooterLink>
                            <FooterLink attr:href="#">Cookies</FooterLink>
                            <FooterLink attr:href="#">Security</FooterLink>
                        </FooterLinksSection>
                    </FooterSectionsGrid>
                </FooterGrid>
                <FooterSection class="border-t">
                    <FooterCopyright class="block order-last text-center md:order-first">
                        Rustify, All rights reserved
                    </FooterCopyright>
                    <FooterSocialContainer class="order-first justify-center md:order-last">
                        <FooterExternalLink href="#" attr:aria-label="Twitter">
                            <LogoTwitter />
                        </FooterExternalLink>
                        <FooterExternalLink href="#" attr:aria-label="LinkedIn">
                            <LogoLinkedIn />
                        </FooterExternalLink>
                        <FooterExternalLink href="#" attr:aria-label="Facebook">
                            <LogoFacebook />
                        </FooterExternalLink>
                        <FooterExternalLink href="#" attr:aria-label="Threads">
                            <LogoThreads />
                        </FooterExternalLink>
                        <FooterExternalLink href="#" attr:aria-label="Instagram">
                            <LogoInstagram />
                        </FooterExternalLink>
                        <FooterExternalLink href="#" attr:aria-label="TikTok">
                            <LogoTikTok />
                        </FooterExternalLink>
                    </FooterSocialContainer>
                </FooterSection>
            </FooterContainer>
        </Footer>
    }
}
