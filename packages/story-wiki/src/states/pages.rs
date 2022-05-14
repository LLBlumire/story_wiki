use std::collections::HashMap;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::utils::{downloadable_resource::DownloadableResource, fetch::fetch_text, irc::Irc};

#[derive(Default, Clone, PartialEq, Store)]
struct State {
    /// Key is resource path, value is downloaded content
    resource: HashMap<String, DownloadableResource<Irc<String>>>,
}

#[hook]
pub fn use_page(resource_path: &str) -> DownloadableResource<Irc<String>> {
    let (state, dispatch) = use_store::<State>();
    let resource_path = resource_path.to_string();

    match state
        .resource
        .get(&resource_path)
        .cloned()
        .unwrap_or_default()
    {
        DownloadableResource::Downloading => DownloadableResource::Downloading,
        DownloadableResource::NotYetRequested => {
            log::debug!("Downloading page {resource_path}");
            let resource_path_ins = resource_path.clone();
            dispatch.reduce(move |state| {
                state
                    .resource
                    .insert(resource_path_ins, DownloadableResource::Downloading);
            });
            spawn_local(async move {
                if let Some(page) = fetch_text(&resource_path).await {
                    log::debug!("Page {resource_path} downloaded");
                    dispatch.reduce(move |state| {
                        state
                            .resource
                            .insert(resource_path, DownloadableResource::Ready(Irc::new(page)))
                    });
                } else {
                    log::error!("Error downloading or deserializing page {resource_path}");
                }
            });
            DownloadableResource::Downloading
        }
        DownloadableResource::Ready(ready) => DownloadableResource::Ready(ready),
    }
}
