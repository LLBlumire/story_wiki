use yew::prelude::*;
use yew_router::prelude::*;

pub mod not_found;
pub mod page;
pub mod root;
pub mod search;

use not_found::RouteNotFound;
use page::RoutePage;
use root::RouteRoot;
use search::RouteSearch;

/// All the possible routes for our application
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,

    #[at("/page/:page_reference")]
    DefaultContinuityPage {
        page_reference: String,
    },

    #[at("/:continuity_reference/page/:page_reference")]
    Page {
        continuity_reference: String,
        page_reference: String,
    },


    #[at("/search")]
    DefaultContinuitySearch,

    #[at("/:continuity_reference/search")]
    Search { continuity_reference: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}
impl Route {
    pub fn continuity(&self) -> Option<&str> {
        match self {
            Route::Page {
                continuity_reference,
                ..
            }
            | Route::Search {
                continuity_reference,
            } => Some(continuity_reference),
            _ => None,
        }
    }

    pub fn with_continuity(mut self, new_continuity: String) -> Self {
        if let Route::Page {
            continuity_reference,
            ..
        }
        | Route::Search {
            continuity_reference,
        } = &mut self
        {
            *continuity_reference = new_continuity;
        } else if let Route::DefaultContinuityPage { page_reference } = self {
            self = Route::Page { page_reference, continuity_reference: new_continuity }
        }
        self
    }
}

pub fn switch(routes: &Route) -> Html {
    log::trace!("Rendering Route Switcher");
    match routes.clone() {
        Route::Root => html! { <RouteRoot /> },
        Route::DefaultContinuityPage { page_reference } |
        Route::Page { page_reference, .. } => html! { <RoutePage {page_reference} /> },
        Route::DefaultContinuitySearch | Route::Search { .. } => html! { <RouteSearch /> },
         Route::NotFound => html! { <RouteNotFound />},
    }
}
