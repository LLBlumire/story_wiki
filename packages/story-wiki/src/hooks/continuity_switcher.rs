use yew::prelude::*;
use yew_router::history::HistoryResult;
use yew_router::prelude::*;

use crate::routes::search::SearchQuery;
use crate::routes::Route;
use crate::states::manifest::{use_manifest, Continuity, Manifest};
use crate::utils::downloadable_resource::DownloadableResource;
use crate::utils::irc::Irc;

#[derive(Debug, Clone)]
pub struct ContinuitySwithcerHandle {
    navigator: Navigator,
    route: Route,
    location: Location,
}

impl ContinuitySwithcerHandle {
    pub fn switch(&self, new_continuity: &Continuity) -> HistoryResult<()> {
        match self.route.clone().with_continuity(new_continuity) {
            new_route @ (Route::Root
            | Route::NotFound
            | Route::Page { .. }
            | Route::DefaultContinuityPage { .. } 
            | Route::Categories { .. } 
            | Route::DefaultContinuityCategories) => {
                self.navigator.push(new_route);
                Ok(())
            }
            new_route @ (Route::Search { .. } | Route::DefaultContinuitySearch) => self
                .navigator
                .push_with_query(new_route, self.location.query::<SearchQuery>()?),
        }
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
    pub fn active(&self) -> Option<&Continuity> {
        let manifest = self.manifest.as_deref().opt()?;
        let from_route = self
            .route
            .as_ref()
            .and_then(Route::continuity_url_prefix)
            .and_then(|continuity_url_prefix| {
                manifest.continuity_from_url_prefix(continuity_url_prefix)
            });

        let default_continuity = || {
            self.manifest
                .as_deref()
                .opt()
                .and_then(Manifest::default_continuity)
        };

        from_route.or_else(default_continuity)
    }

    pub fn is_from_route(&self) -> bool {
        self.route
            .as_ref()
            .and_then(Route::continuity_url_prefix)
            .is_some()
    }
}

#[hook]
pub fn use_active_continuity() -> ActiveContinuityHandle {
    let route = use_route();
    let manifest = use_manifest();

    ActiveContinuityHandle { route, manifest }
}
