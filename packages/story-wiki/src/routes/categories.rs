use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{states::{manifest::{use_manifest, Page}, title::use_title_switcher, active_release::use_active_release_tracker}, hooks::continuity_switcher::use_active_continuity, routes::Route};


#[function_component]
pub fn RouteCategories() -> Html {
    log::trace!("Rendering RouteCategories");

    let manifest = use_manifest();
    let title_switcher = use_title_switcher();
    let active_release_tracker = use_active_release_tracker();
    let active_continuity = use_active_continuity();
    let navigator = use_navigator();

    let continuity = active_continuity.active().unwrap();
    let manifest = manifest.unwrap();
    let navigator = navigator.unwrap();

    let manifest_has_multiple_continuities = manifest.has_multiple_continuities();
    let continuity_in_url = active_continuity.is_from_route();
    if !manifest_has_multiple_continuities && continuity_in_url {
        navigator.replace(Route::DefaultContinuityCategories);
    }
    if manifest_has_multiple_continuities && !continuity_in_url {
        navigator.replace(Route::Categories {
            continuity_url_prefix: continuity.url_prefix().to_string(),
        })
    }

    title_switcher.page("Categories".into());

    let observed_releases_references =
        active_release_tracker.observed_releases_references(&manifest);


    let mut categories = HashMap::<String, Vec<&Page>>::new();
    for page in manifest.pages(continuity.reference_name()) {
        if page.should_show(&observed_releases_references, continuity.prefix()) {
            for category in page.categories(&observed_releases_references, continuity.prefix()) {
                categories.entry(category).or_default().push(page);
            }
        }
    }
    let mut ordered_categories = categories.into_iter().collect::<Vec<_>>();
    ordered_categories.sort_by_key(|(category, _)| category.clone());
    
    let out = ordered_categories.into_iter().map(|(category, mut pages)| {
        pages.sort_by_key(|page| page.display_name().to_string());
        let pages_list = pages.into_iter().map(|page| {
            html! {
                <li class="category-entry">
                    <Link<Route> to={
                        Route::Page { continuity_url_prefix: continuity.url_prefix().to_string(), page_reference: page.page_url().to_string() }
                    }>
                        { page.display_name() }
                    </Link<Route>>
                </li>
            }
        });
        html! {
            <>
                <h2>{category}</h2>
                <ul class="noindent">
                { for pages_list }
                </ul>
            </>
        }
    });

    html! {
        <main>
            <article>
                { for out }
            </article>
        </main>
    }
}