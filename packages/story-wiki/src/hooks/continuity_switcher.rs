use story_wiki_core::manifest::Manifest;
use yew::prelude::*;
use yew_router::{history::HistoryResult, prelude::*};

use crate::{
    routes::{search::SearchQuery, Route},
    states::manifest::use_manifest,
    utils::{downloadable_resource::DownloadableResource, irc::Irc},
};

#[derive(Debug, Clone)]
pub struct ContinuitySwithcerHandle {
    navigator: Navigator,
    route: Route,
    location: Location,
}

impl ContinuitySwithcerHandle {
    pub fn switch(self, new_continuity: String) -> HistoryResult<()> {
        match self.route.with_continuity(new_continuity) {
            new_route @ (Route::Root
            | Route::NotFound
            | Route::Page { .. }
            | Route::DefaultContinuityPage { .. }) => {
                self.navigator.push(new_route);
                Ok(())
            }
            new_route @ (Route::Search { .. } | Route::DefaultContinuitySearch) => self
                .navigator
                .push_with_query(new_route, self.location.query::<SearchQuery>()?),
        }
    }
    pub fn active(&self) -> Option<&str> {
        self.route.continuity()
    }
}

#[hook]
pub fn use_continuity_switcher() -> Option<ContinuitySwithcerHandle> {
    let navigator = use_navigator()?;
    let route = use_route()?;
    let location = use_location()?;

    Some(ContinuitySwithcerHandle {
        navigator,
        route,
        location,
    })
}

pub struct ActiveContinuityHandle {
    route: Option<Route>,
    manifest: DownloadableResource<Irc<Manifest>>,
}
impl ActiveContinuityHandle {
    pub fn active(&self) -> Option<&str> {
        let from_route = self.route.as_ref().and_then(Route::continuity);
        let from_manifest = || {
            self.manifest
                .as_deref()
                .opt()
                .and_then(Manifest::default_continuity)
                .map(|c| c.reference_name.as_str())
        };
        from_route.or_else(from_manifest)
    }

    pub fn is_from_route(&self) -> bool {
        self.route.as_ref().and_then(Route::continuity).is_some()
    }
}

#[hook]
pub fn use_active_continuity() -> ActiveContinuityHandle {
    let route = use_route();
    let manifest = use_manifest();

    ActiveContinuityHandle { route, manifest }
}
