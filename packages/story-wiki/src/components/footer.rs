use yew::prelude::*;

use crate::states::release_citations::use_release_citations_toggle;

/// Renders the Footer of StoryWiki
#[function_component]
pub fn Footer() -> Html {
    log::trace!("Rendering Footer");

    let release_citations_toggler = use_release_citations_toggle();

    html! {
        <footer>
            <a
                href="https://github.com/llblumire/story-wiki"
                target="_blank"
                rel="noopener noreferrer"
            >
                {"Powered by StoryWiki"}
            </a>
            <button onclick={release_citations_toggler.toggle()}>
                {"Toggle Release Citations"}
            </button>
        </footer>
    }
}
