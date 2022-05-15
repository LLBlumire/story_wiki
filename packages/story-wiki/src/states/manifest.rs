use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;

use convert_case::{Case, Casing};
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::routes::search::tokenize;
use crate::utils::cond::should_show;
use crate::utils::downloadable_resource::DownloadableResource;
use crate::utils::fetch::fetch_manifest;
use crate::utils::irc::Irc;

/// The site manifest file
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Manifest {
    /// The title of the website
    title: String,
    /// Configuration for each continuity
    #[serde(default)]
    continuities: Vec<Continuity>,
    /// Configuration for each release, keys must be continuity `reference_name`s
    #[serde(default)]
    releases: HashMap<String, Vec<Release>>,
    /// Configuration for each page, keys must be continuity `reference_name`s
    #[serde(default)]
    pages: HashMap<String, Vec<Page>>,
}

impl Manifest {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn continuities(&self) -> &[Continuity] {
        &self.continuities
    }

    pub fn default_continuity(&self) -> Option<&Continuity> {
        self.continuities().first()
    }

    pub fn has_multiple_continuities(&self) -> bool {
        self.continuities().len() != 1
    }

    pub fn continuity(&self, continuity_reference: &str) -> Option<&Continuity> {
        self.continuities()
            .iter()
            .find(|continuity| &continuity.reference_name == continuity_reference)
    }

    pub fn releases(&self, continuity_reference: &str) -> &[Release] {
        self.releases
            .get(continuity_reference)
            .map(|r| &r[..])
            .unwrap_or(NO_RELEASES)
    }

    pub fn default_release(&self, continuity_reference: &str) -> Option<&Release> {
        self.releases(continuity_reference).first()
    }

    pub fn all_releases(&self) -> Vec<(&Release, &Continuity)> {
        self.continuities()
            .iter()
            .flat_map(|continuity| {
                self.releases(&continuity.reference_name)
                    .iter()
                    .zip(repeat(continuity))
            })
            .collect()
    }

    pub fn release(&self, continuity_reference: &str, release_reference: &str) -> Option<&Release> {
        self.releases(continuity_reference)
            .iter()
            .find(|release| &release.reference_name == release_reference)
    }

    pub fn has_multiple_releases(&self) -> bool {
        self.continuities()
            .iter()
            .any(|continuity| self.releases(&continuity.reference_name).len() != 1)
    }

    pub fn pages(&self, continuity_reference: &str) -> &[Page] {
        self.pages
            .get(continuity_reference)
            .map(|p| &p[..])
            .unwrap_or(NO_PAGES)
    }

    pub fn default_page(&self, continuity_reference: &str) -> Option<&Page> {
        self.pages(continuity_reference).first()
    }

    pub fn page(&self, continuity_reference: &str, page_reference: &str) -> Option<&Page> {
        self.pages(continuity_reference)
            .iter()
            .find(|page| &page.page_url == page_reference)
    }
    pub fn continuity_from_url_prefix(&self, url_prefix: &str) -> Option<&Continuity> {
        self.continuities()
            .iter()
            .find(|continuity| continuity.url_prefix() == url_prefix)
    }
}

/// Configuration for a single continuity
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Continuity {
    /// The name to refer to the continuity as in configuration
    reference_name: String,
    /// The name to display in the picker and title
    display_name: Option<String>,
    /// The path to prefix in the URL
    url_prefix: Option<String>,
    /// The prefix for releases of this continuity
    prefix: String,
}

impl Continuity {
    pub fn reference_name(&self) -> &str {
        &self.reference_name
    }

    pub fn display_name(&self) -> Cow<str> {
        self.display_name
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(self.reference_name().to_case(Case::Title)))
    }

    pub fn url_prefix(&self) -> Cow<str> {
        self.url_prefix
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(self.reference_name().to_string()))
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

/// Configuration for a single release
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Release {
    /// The name to refer to the continuity as in configuration and conditionals
    reference_name: String,
    /// The name to display in the picker
    display_name: Option<String>,
    /// An optional group heading this and all releases after should be part of
    begins_group: Option<String>,
}

impl Release {
    pub fn reference_name(&self) -> &str {
        &self.reference_name
    }

    pub fn display_name(&self) -> Cow<str> {
        self.display_name
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(self.reference_name().to_case(Case::Title)))
    }

    pub fn begins_group(&self) -> Option<Option<&str>> {
        if let Some(group) = self.begins_group.as_deref() {
            if group.is_empty() {
                Some(None)
            } else {
                Some(Some(group))
            }
        } else {
            None
        }
    }
}

/// Configuration for a single page
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Page {
    /// The name to use in the URL and in document links
    page_url: String,
    /// The path to download the pages markdown
    resource_path: String,
    /// The name to display in the title
    display_name: Option<String>,
    /// Configures conditions which must all be met in order to show the page.
    /// i.e. ["o-b2", "x-b5"] would only show after b2 but before b5.
    #[serde(default)]
    show_cond: Vec<String>,
    /// Terms that relate to the page that should be used for searching
    #[serde(default)]
    keywords: String,
    /// Terms that should only relate to the page given a condition
    #[serde(default)]
    keywords_cond: Vec<KeywordsCond>,
    /// Other reference names that should redirect to this one
    #[serde(default)]
    title_peers: Vec<String>,
    /// Title peers that should only redirect after a certain condition
    #[serde(default)]
    title_peers_cond: Vec<TitlePeerCond>,
    /// Categories that the page should show in, if none are configured it will
    /// show as "Uncategorized".
    #[serde(default)]
    categories: Vec<String>,
    /// Categories that the page should only show in within a specific condition
    #[serde(default)]
    categories_cond: Vec<CategoryCond>,
}

/// A conditional keywords set
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct KeywordsCond {
    /// The keywords that should be assigned
    keywords: String,
    /// The conditions that must be assign the keywords
    cond: Vec<String>,
}

/// A conditional title peer
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct TitlePeerCond {
    /// The title which should redirect
    peer: String,
    /// The conditions that must be try to do the redirect
    cond: Vec<String>,
}

/// A conditional category
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CategoryCond {
    /// The category to be included in
    category: String,
    /// The conditions that must be try to show in that category
    cond: Vec<String>,
}

static NO_RELEASES: &[Release] = &[];
static NO_PAGES: &[Page] = &[];

impl Page {
    pub fn page_url(&self) -> &str {
        &self.page_url
    }
    pub fn resource_path(&self) -> &str {
        &self.resource_path
    }
    pub fn display_name(&self) -> Cow<str> {
        self.display_name
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(self.page_url().to_case(Case::Title)))
    }
    pub fn should_show(
        &self,
        observed_releases_references: &HashSet<&str>,
        continuity_prefix: &str,
    ) -> bool {
        self.show_cond
            .iter()
            .all(|cond| should_show(observed_releases_references, cond, continuity_prefix))
    }
    pub fn keywords(
        &self,
        observed_releases_references: &HashSet<&str>,
        continuity_prefix: &str,
    ) -> HashSet<String> {
        tokenize(&self.keywords)
            .chain(
                self.keywords_cond
                    .iter()
                    .filter(|keyword_cond| {
                        keyword_cond.cond.iter().all(|cond| {
                            should_show(observed_releases_references, cond, continuity_prefix)
                        })
                    })
                    .flat_map(|keyword_cond| tokenize(&keyword_cond.keywords)),
            )
            .collect()
    }
    pub fn title_peers(&self, observed_releases_references: &HashSet<&str>, continuity_prefix: &str) -> Vec<String> {
        self.title_peers.clone().into_iter().chain(
            self.title_peers_cond.iter().filter(|title_peer_cond| {
                title_peer_cond.cond.iter().all(|cond| {
                    should_show(observed_releases_references, cond, continuity_prefix)
                })
            }).map(|title_peer_cond| title_peer_cond.peer.clone())
        ).collect()
    }
    pub fn categories(&self, observed_releases_references: &HashSet<&str>, continuity_prefix: &str) -> Vec<String> {
        self.categories.clone().into_iter().chain(
            self.categories_cond.iter().filter(|category_cond| {
                category_cond.cond.iter().all(|cond| {
                    should_show(observed_releases_references, cond, continuity_prefix)
                })
            }).map(|title_peer_cond| title_peer_cond.category.clone())
        ).collect()
    }
}

#[derive(Default, Clone, PartialEq, Store)]
struct State {
    resource: DownloadableResource<Irc<Manifest>>,
}

/// Downloads the manifest, and returns it, or if it has already been downloaded
/// simply returns it.
#[hook]
pub fn use_manifest() -> DownloadableResource<Irc<Manifest>> {
    let (state, dispatch) = use_store::<State>();

    use_effect_with_deps(
        move |_| {
            if !dispatch.get().resource.requested() {
                log::debug!("Downloading and deserializing manifest");
                dispatch.set(State {
                    resource: DownloadableResource::Downloading,
                });
                spawn_local(async move {
                    if let Some(manifest) = fetch_manifest().await {
                        log::debug!("Manifest downloaded and deserialized");
                        dispatch.set(State {
                            resource: DownloadableResource::Ready(Irc::new(manifest)),
                        });
                    } else {
                        log::error!("Error downloading or deserializing manifest");
                    }
                });
            }
            || ()
        },
        (),
    );

    state.resource.clone()
}
