use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::routes::Route;
use crate::states::manifest::use_manifest;

#[function_component]
pub fn RouteRoot() -> Html {
    log::trace!("Rendering RouteRoot");

    let navigator = use_navigator();
    let manifest = use_manifest();

    let navigator = navigator.unwrap();
    let manifest = manifest.unwrap();

    let continuity = manifest.default_continuity();
    let continuity_url_prefix = continuity.map(|c| c.url_prefix().to_string());
    let continuity_reference_name = continuity.map(|c| c.reference_name());

    let default_page = continuity_reference_name
        .as_deref()
        .and_then(|default_continuity| manifest.default_page(default_continuity))
        .map(|p| p.page_url().to_string());

    if let Some((continuity_url_prefix, page_reference)) = continuity_url_prefix.zip(default_page) {
        navigator.replace(Route::Page {
            continuity_url_prefix,
            page_reference,
        });
    }

    html! {
        <main><p>{"StoryWiki has not configured any continuities or pages"}</p></main>
    }
}
