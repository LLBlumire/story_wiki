use story_wiki_core::manifest::Manifest;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::utils::{downloadable_resource::DownloadableResource, fetch::fetch_manifest, irc::Irc};

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
