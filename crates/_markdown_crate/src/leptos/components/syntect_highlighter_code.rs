use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::highlight_code::highlight_code;
use crate::highlight_language::HighlightLanguage;

#[component]
pub fn SyntectHighlighterCode(
    #[prop(into)] code: String,
    language: HighlightLanguage,
    #[prop(optional, into)] class: String,
    #[prop(default = true)] show_scrollbar_on_hover: bool,
) -> impl IntoView {
    let merged_class = tw_merge!(
        if show_scrollbar_on_hover { "scrollbar__on_hover" } else { "" },
        "h-full max-h-[370px] overflow-y-auto overscroll-y-contain",
        "whitespace-pre-wrap p-4 [&_span]:text-xs rounded-md bg-muted",
        class
    );

    view! {
        <div class="group/scrollbar-on-hover">
            <pre data-name="__SyntectHighlighterCode" class=merged_class>
                <code inner_html=highlight_code(&code, Some(language.as_ref()), None) />
            </pre>
        </div>
    }
}
