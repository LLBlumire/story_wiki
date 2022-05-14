use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::components::page_render::PageRender;
use crate::hooks::continuity_switcher::use_active_continuity;
use crate::routes::search::SearchQuery;
use crate::routes::Route;
use crate::states::active_release::use_active_release_tracker;
use crate::states::manifest::use_manifest;
use crate::states::title::use_title_switcher;

#[derive(PartialEq, Properties)]
pub struct RoutePageProps {
    pub page_reference: String,
}

#[function_component]
pub fn RoutePage(props: &RoutePageProps) -> Html {
    log::trace!("Rendering RoutePage `{}`", props.page_reference);

    let manifest = use_manifest();
    let title_switcher = use_title_switcher();
    let active_release_tracker = use_active_release_tracker();
    let active_continuity = use_active_continuity();
    let navigator = use_navigator();

    let continuity_reference = active_continuity.active().unwrap();
    let manifest = manifest.unwrap();
    let navigator = navigator.unwrap();

    let manifest_has_multiple_continuities = manifest.has_multiple_continuities();
    let continuity_in_url = active_continuity.is_from_route();
    if !manifest_has_multiple_continuities && continuity_in_url {
        navigator.replace(Route::DefaultContinuityPage {
            page_reference: props.page_reference.clone(),
        });
    }
    if manifest_has_multiple_continuities && !continuity_in_url {
        navigator.replace(Route::Page {
            continuity_reference: continuity_reference.to_string(),
            page_reference: props.page_reference.clone(),
        })
    }

    let page = manifest.page(&continuity_reference, &props.page_reference);

    let observed_releases_references =
        active_release_tracker.observed_releases_references(&manifest);

    match page {
        None => html! { <main>{"Page not found."}</main> },
        Some(page) => {
            if !page.should_show(&observed_releases_references) {
                log::trace!("Page not available on active release");
                navigator
                    .replace_with_query(
                        Route::Search {
                            continuity_reference: continuity_reference.to_string(),
                        },
                        SearchQuery {
                            query: props.page_reference.clone(),
                        },
                    )
                    .unwrap();
            }
            title_switcher.page(page.display_name.to_string());
            html! {
                <main>
                    <PageRender
                        resource_path={page.resource_path.clone()}
                        continuity={continuity_reference.to_string()}
                    />
                </main>
            }
        }
    }
}
