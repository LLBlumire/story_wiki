use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::Once,
};

use gloo_events::EventListener;
use serde::{Deserialize, Serialize};
use story_wiki_core::manifest::Manifest;
use yew::prelude::*;
use yewdux::{
    prelude::*,
    storage::{load, save, Area},
};

use crate::{
    states::manifest::use_manifest,
    utils::{downloadable_resource::DownloadableResource, irc::Irc},
};

static STATE_LISTENER_ON: Once = Once::new();

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
struct State {
    /// Map from `Continuity.reference_name` to `Release.reference_name`
    releases: HashMap<String, String>,
}

impl Store for State {
    fn new() -> Self {
        STATE_LISTENER_ON.call_once(|| {
            EventListener::new(&web_sys::window().unwrap(), "storage", move |_| {
                log::debug!("Received storage event");
                Dispatch::<State>::new()
                    .reduce(|state| *state = load(Area::Local).unwrap().unwrap());
            })
            .forget();
        });
        load(Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }

    fn changed(&mut self) {
        save(self, Area::Local).expect("Unable to save state");
    }
}

pub struct ReleaseTrackerHandle {
    state: Rc<State>,
    manifest: DownloadableResource<Irc<Manifest>>,
}
impl ReleaseTrackerHandle {
    pub fn active(&self, continuity_reference: &str) -> Option<&str> {
        let get_default = || {
            self.manifest
                .as_ref()
                .opt()
                .and_then(|m| m.default_release(continuity_reference))
                .map(|r| r.reference_name.as_str())
        };
        self.state
            .releases
            .get(continuity_reference)
            .map(|r| r.as_str())
            .or_else(get_default)
    }

    pub fn observed_releases_references<'a>(&self, manifest: &'a Manifest) -> HashSet<&'a str> {
        manifest
            .continuities()
            .iter()
            .flat_map(|continuity| {
                let active_release = self.active(&continuity.reference_name);
                active_release.into_iter().flat_map(move |active_release| {
                    let mut seen = false;
                    manifest
                        .releases(&continuity.reference_name)
                        .iter()
                        .take_while(move |release| {
                            let was_seen = seen;
                            seen = &release.reference_name == active_release;
                            !was_seen
                        })
                })
            })
            .map(|release| &release.reference_name[..])
            .collect()
    }
}

pub struct ReleaseSwitcherHandle {
    dispatch: Dispatch<State>,
}
impl ReleaseSwitcherHandle {
    pub fn switch(&self, continuity_reference: String, release_reference: String) {
        self.dispatch.reduce(move |state| {
            state
                .releases
                .insert(continuity_reference, release_reference)
        })
    }
}

#[hook]
pub fn use_active_release_switcher() -> ReleaseSwitcherHandle {
    ReleaseSwitcherHandle {
        dispatch: Dispatch::new(),
    }
}

#[hook]
pub fn use_active_release_tracker() -> ReleaseTrackerHandle {
    let manifest = use_manifest();
    let state = use_store_value();
    ReleaseTrackerHandle { state, manifest }
}
