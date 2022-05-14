use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use serde::Deserialize;

use crate::cond::should_show;

/// The site manifest file
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Manifest {
    /// The title of the website
    pub title: String,
    /// Configuration for each continuity
    #[serde(default)]
    pub continuities: Vec<Continuity>,
    /// Configuration for each release, keys must be continuity `reference_name`s
    #[serde(default)]
    pub releases: HashMap<String, Vec<Release>>,
    /// Configuration for each page, keys must be continuity `reference_name`s
    #[serde(default)]
    pub pages: HashMap<String, Vec<Page>>,
}

/// Configuration for a single continuity
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Continuity {
    /// The name to display in the picker and title
    pub display_name: String,
    /// The name to refer to the continuity as in configuration
    pub reference_name: String,
}

/// Configuration for a single release
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Release {
    /// The name to display in the picker
    pub display_name: String,
    /// The name to refer to the continuity as in configuration and conditionals
    pub reference_name: String,
    /// An optional group heading this and all releases after should be part of
    pub begins_group: Option<String>,
}

/// Configuration for a single page
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Page {
    /// The name to display in the title
    pub display_name: String,
    /// The name to use in the URL and in document links
    pub reference_name: String,
    /// The path to download the pages markdown
    pub resource_path: String,
    /// Configures conditions which must all be met in order to show the page.
    /// i.e. ["o-b2", "x-b5"] would only show after b2 but before b5.
    #[serde(default)]
    pub show_cond: Vec<String>,
    /// Terms that relate to the page that should be used for searching
    #[serde(default)]
    pub keywords: String,
    /// Terms that should only relate to the page given a condition
    #[serde(default)]
    pub keywords_cond: Vec<KeywordsCond>,
    /// Other reference names that should redirect to this one
    #[serde(default)]
    pub title_peers: Vec<String>,
    /// Title peers that should only redirect after a certain condition
    #[serde(default)]
    pub title_peers_cond: Vec<TitlePeerCond>,
    /// Categories that the page should show in, if none are configured it will
    /// show as "Uncategorized".
    #[serde(default)]
    pub categories: Vec<String>,
    /// Categories that the page should only show in within a specific condition
    #[serde(default)]
    pub categories_cond: Vec<CategoryCond>,
}

/// A conditional keywords set
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct KeywordsCond {
    /// The keywords that should be assigned
    pub keywords: String,
    /// The conditions that must be assign the keywords
    pub cond: Vec<String>,
}

/// A conditional title peer
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct TitlePeerCond {
    /// The title which should redirect
    pub peer: String,
    /// The conditions that must be try to do the redirect
    pub cond: Vec<String>,
}

/// A conditional category
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CategoryCond {
    /// The category to be included in
    pub category: String,
    /// The conditions that must be try to show in that category
    pub cond: Vec<String>,
}

static NO_RELEASES: &[Release] = &[];
static NO_PAGES: &[Page] = &[];

impl Manifest {
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
            .find(|page| &page.reference_name == page_reference)
    }
}

impl Page {
    pub fn should_show(&self, observed_releases_references: &HashSet<&str>) -> bool {
        self.show_cond
            .iter()
            .all(|cond| should_show(observed_releases_references, cond))
    }
}
