use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

use crate::components::picker::continuity_picker::ContinuityPicker;
use crate::components::picker::release_picker::ReleasePicker;
use crate::hooks::continuity_switcher::use_active_continuity;
use crate::routes::search::SearchQuery;
use crate::routes::Route;
use crate::states::manifest::use_manifest;

/// Renders the header of StoryWiki
///
/// # Panics
///  - If the manifest is not loaded
///  - If the navigator is not available
///  - If there is no configured default continuity and the page is not one with
///    a set continuity.
#[function_component]
pub fn Header() -> Html {
    log::trace!("Rendering Header");

    let manifest = use_manifest();
    let search_node_ref = use_node_ref();
    let navigator = use_navigator();
    let active_continuity = use_active_continuity();
    let location = use_location();

    let manifest = manifest.unwrap();
    let navigator = navigator.unwrap();
    let continuity_reference = active_continuity.active().unwrap().to_string();

    let onsubmit = {
        let search_node_ref = search_node_ref.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let search_for = search_node_ref.cast::<HtmlInputElement>().unwrap().value();
            navigator
                .push_with_query(
                    Route::Search {
                        continuity_reference: continuity_reference.clone(),
                    },
                    SearchQuery { query: search_for },
                )
                .unwrap();
        })
    };

    let search_query = location
        .as_ref()
        .and_then(|location| location.query::<SearchQuery>().ok());
    let search_query = search_query.map(|search| search.query);
    {
        let search = search_node_ref.clone();
        use_effect_with_deps(
            move |search_query| {
                let search = search.cast::<HtmlInputElement>().unwrap();
                if let Some(query) = search_query.as_deref() {
                    search.set_value(query);
                } else {
                    search.set_value("");
                }
                || ()
            },
            search_query,
        );
    }

    html! {
        <header>
            <span class="site-name">
                {&manifest.title}
            </span>
            <form {onsubmit}>
                <input type="search" placeholder="Search" ref={search_node_ref}/>
                <input type="submit" value="?" />
            </form>
            if manifest.has_multiple_continuities() {
                <ContinuityPicker />
            }
            if manifest.has_multiple_releases() {
                <ReleasePicker />
            }
        </header>
    }
}
