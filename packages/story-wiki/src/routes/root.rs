use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{routes::Route, states::manifest::use_manifest};

#[function_component]
pub fn RouteRoot() -> Html {
    log::trace!("Rendering RouteRoot");

    let navigator = use_navigator();
    let manifest = use_manifest();

    let navigator = navigator.unwrap();
    let manifest = manifest.unwrap();

    let default_continuity = manifest
        .default_continuity()
        .map(|c| c.reference_name.to_string());
    let default_page = default_continuity
        .as_deref()
        .and_then(|default_continuity| manifest.default_page(default_continuity))
        .map(|p| p.reference_name.to_string());

    if let Some((continuity_reference, page_reference)) = default_continuity.zip(default_page) {
        navigator.replace(Route::Page {
            continuity_reference,
            page_reference,
        });
    }

    html! {
        <main><p>{"StoryWiki has not configured any continuities or pages"}</p></main>
    }
}
