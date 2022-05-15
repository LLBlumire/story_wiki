use yew::prelude::*;
use yew_router::prelude::*;

pub mod not_found;
pub mod page;
pub mod root;
pub mod search;
pub mod categories;

use not_found::RouteNotFound;
use page::RoutePage;
use root::RouteRoot;
use search::RouteSearch;
use categories::RouteCategories;

use crate::states::manifest::Continuity;

/// All the possible routes for our application
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,

    #[at("/page/:page_reference")]
    DefaultContinuityPage { page_reference: String },

    #[at("/:continuity_url_prefix/page/:page_reference")]
    Page {
        continuity_url_prefix: String,
        page_reference: String,
    },

    #[at("/search")]
    DefaultContinuitySearch,

    #[at("/:continuity_url_prefix/search")]
    Search { continuity_url_prefix: String },

    #[at("/categories")]
    DefaultContinuityCategories,

    #[at("/:continuity_url_prefix/categories")]
    Categories { continuity_url_prefix: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}
impl Route {
    pub fn continuity_url_prefix(&self) -> Option<&str> {
        match self {
            Route::Page {
                continuity_url_prefix,
                ..
            }
            | Route::Search {
                continuity_url_prefix,
            }
            | Route::Categories { continuity_url_prefix } => Some(continuity_url_prefix.as_str()),
            _ => None,
        }
    }

    pub fn with_continuity(mut self, new_continuity: &Continuity) -> Self {
        if let Route::Page {
            continuity_url_prefix,
            ..
        }
        | Route::Search {
            continuity_url_prefix,
        } | Route::Categories { continuity_url_prefix } = &mut self
        {
            *continuity_url_prefix = new_continuity.url_prefix().to_string();
        } else if let Route::DefaultContinuityPage { page_reference } = self {
            self = Route::Page {
                page_reference,
                continuity_url_prefix: new_continuity.url_prefix().to_string(),
            }
        }
        self
    }
}

pub fn switch(routes: &Route) -> Html {
    log::trace!("Rendering Route Switcher");
    match routes.clone() {
        Route::Root => html! { <RouteRoot /> },
        Route::DefaultContinuityPage { page_reference } | Route::Page { page_reference, .. } => {
            html! { <RoutePage {page_reference} /> }
        }
        Route::DefaultContinuitySearch | Route::Search { .. } => html! { <RouteSearch /> },
        Route::DefaultContinuityCategories | Route::Categories { .. } => html! { <RouteCategories /> },
        Route::NotFound => html! { <RouteNotFound />},
    }
}
