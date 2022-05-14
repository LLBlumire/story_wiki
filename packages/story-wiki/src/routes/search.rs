use std::collections::{HashMap, HashSet};
use std::iter::empty;

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::components::Link;
use yew_router::hooks::{use_location, use_navigator};

use crate::hooks::continuity_switcher::use_active_continuity;
use crate::routes::Route;
use crate::states::active_release::use_active_release_tracker;
use crate::states::manifest::{use_manifest, CategoryCond, KeywordsCond, TitlePeerCond};
use crate::states::title::use_title_switcher;
use crate::utils::cond::should_show;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    #[serde(rename = "q")]
    pub query: String,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct SearchResult {
    title: String,
    reference_name: String,
}

#[function_component]
pub fn RouteSearch() -> Html {
    log::trace!("Rendering RouteSearch");

    let location = use_location();
    let navigator = use_navigator();
    let title = use_title_switcher();
    let manifest = use_manifest();
    let release_tracker = use_active_release_tracker();
    let active_continuity = use_active_continuity();

    let manifest = manifest.unwrap();
    let continuity_reference = active_continuity.active().unwrap();
    let navigator = navigator.unwrap();

    let manifest_has_multiple_continuities = !manifest.has_multiple_continuities();
    let continuity_in_url = active_continuity.is_from_route();
    if !manifest_has_multiple_continuities && continuity_in_url {
        navigator.replace(Route::DefaultContinuitySearch);
    }
    if manifest_has_multiple_continuities && !continuity_in_url {
        navigator.replace(Route::Search {
            continuity_reference: continuity_reference.to_string(),
        })
    }

    let search = location
        .as_ref()
        .and_then(|location| location.query::<SearchQuery>().ok());

    let search_query = search.map(|search| search.query).unwrap_or_default();

    if search_query.is_empty() {
        if let Some(page) = manifest.default_page(continuity_reference) {
            navigator.replace(Route::Page {
                page_reference: page.reference_name.clone(),
                continuity_reference: continuity_reference.to_string(),
            })
        }
    }

    let mut title_results = HashMap::<SearchResult, usize>::new();
    let mut category_results = HashMap::<SearchResult, usize>::new();
    let mut keyword_results = HashMap::<SearchResult, usize>::new();

    let tokenized_query = tokenize(&cleanup(&search_query)).collect::<Vec<_>>();

    let observed_releases = release_tracker.observed_releases_references(&manifest);

    for page in manifest
        .pages(continuity_reference)
        .iter()
        .filter(|page| page.should_show(&observed_releases))
    {
        let tokenized_page_title = tokenize(&cleanup(&page.display_name)).collect::<Vec<_>>();
        let tokenized_refernece_name = tokenize(&cleanup(&page.reference_name)).collect::<Vec<_>>();
        if tokenized_query == tokenized_page_title || tokenized_query == tokenized_refernece_name {
            navigator.replace(Route::Page {
                page_reference: page.reference_name.clone(),
                continuity_reference: continuity_reference.to_string(),
            });
        }

        let page_title_score = tokenized_query
            .iter()
            .filter(|term| tokenized_page_title.contains(term))
            .count();
        if page_title_score > 0 {
            let entry = title_results
                .entry(SearchResult {
                    reference_name: page.reference_name.clone(),
                    title: page.display_name.clone(),
                })
                .or_default();
            *entry = (*entry).max(page_title_score);
        }
        for title_peer in page
            .title_peers
            .iter()
            .chain(
                page.title_peers_cond
                    .iter()
                    .filter_map(|TitlePeerCond { peer, cond }| {
                        Some(peer).filter(|_| {
                            cond.iter()
                                .all(|cond| should_show(&observed_releases, cond))
                        })
                    }),
            )
        {
            let tokenized_title_peer = tokenize(&cleanup(&title_peer)).collect::<Vec<_>>();
            if tokenized_query == tokenized_title_peer {
                navigator.replace(Route::Page {
                    page_reference: page.reference_name.clone(),
                    continuity_reference: continuity_reference.to_string(),
                });
            }
            let page_title_peer_score = tokenized_query
                .iter()
                .filter(|term| tokenized_title_peer.contains(term))
                .count();
            if page_title_peer_score > 0 {
                let entry = title_results
                    .entry(SearchResult {
                        reference_name: page.reference_name.clone(),
                        title: page.display_name.clone(),
                    })
                    .or_default();
                *entry = (*entry).max(page_title_peer_score);
            }
        }

        for category in page
            .categories
            .iter()
            .chain(
                page.categories_cond
                    .iter()
                    .filter_map(|CategoryCond { category, cond }| {
                        Some(category).filter(|_| {
                            cond.iter()
                                .all(|cond| should_show(&observed_releases, cond))
                        })
                    }),
            )
        {
            let tokenized_category = tokenize(&cleanup(&category)).collect::<Vec<_>>();
            let page_category_score = tokenized_query
                .iter()
                .filter(|term| tokenized_category.contains(term))
                .count();
            if page_category_score > 0 {
                let entry = category_results
                    .entry(SearchResult {
                        reference_name: page.reference_name.clone(),
                        title: page.display_name.clone(),
                    })
                    .or_default();
                *entry = (*entry).max(page_category_score);
            }
        }

        let tokenized_keywords = tokenize(&cleanup(&page.keywords))
            .chain(
                page.keywords_cond
                    .iter()
                    .filter(|KeywordsCond { cond, .. }| {
                        cond.iter()
                            .all(|cond| should_show(&observed_releases, cond))
                    })
                    .flat_map(|KeywordsCond { keywords, .. }| {
                        tokenize(&cleanup(keywords)).collect::<Vec<_>>()
                    }),
            )
            .collect::<HashSet<_>>();
        let page_keyword_score = tokenized_query
            .iter()
            .filter(|term| tokenized_keywords.contains(term.as_str()))
            .count();
        if page_keyword_score > 0 {
            let entry = keyword_results
                .entry(SearchResult {
                    reference_name: page.reference_name.clone(),
                    title: page.display_name.clone(),
                })
                .or_default();
            *entry = (*entry).max(page_keyword_score);
        }
    }

    let order_scaler = tokenized_query.len();
    let category_scaler = order_scaler + 1;
    let title_scaler = (order_scaler * order_scaler) + 1;

    let mut results = HashMap::<SearchResult, usize>::new();

    for (search, score) in empty()
        .chain(
            title_results
                .into_iter()
                .map(|(search, score)| (search, score * title_scaler)),
        )
        .chain(
            category_results
                .into_iter()
                .map(|(search, score)| (search, score * category_scaler)),
        )
        .chain(keyword_results.into_iter())
    {
        let entry = results.entry(search).or_default();
        *entry = (*entry).max(score);
    }

    let mut results = results.into_iter().collect::<Vec<_>>();
    results.sort_by_key(|(_, score)| *score);

    let results = results.into_iter().map(
        |(
            SearchResult {
                title,
                reference_name,
            },
            _,
        )| {
            html! {
                <li class="search-result">
                    <Link<Route>
                        to={
                            Route::Page {
                                page_reference: reference_name.clone(),
                                continuity_reference: continuity_reference.to_string()
                            }
                        }
                    >
                        {title}
                    </Link<Route>>
                </li>
            }
        },
    );

    title.page("Search".to_string());
    log::debug!("Searching for {search_query:?}");

    html! {
        <main>
            <section>
            <h1>{"Search"}</h1>
                <p>{"Searching for: "}<strong>{format!("\"{search_query}\"")}</strong></p>
                <hr />
                { for results }
            </section>
        </main>
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = String> + '_ {
    input
        .split_whitespace()
        .filter(|&word| !word.trim().is_empty())
        .map(str::to_lowercase)
}

pub fn cleanup(s: &str) -> String {
    s.replace(|c: char| !(c.is_alphanumeric() || c == '\''), " ")
}
