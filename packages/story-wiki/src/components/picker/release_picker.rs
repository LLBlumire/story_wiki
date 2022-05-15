use yew::prelude::*;

use crate::components::picker::{OptionSegment, Picker, PickerFeed};
use crate::hooks::continuity_switcher::use_active_continuity;
use crate::states::active_release::{use_active_release_switcher, use_active_release_tracker};
use crate::states::manifest::{use_manifest, Release};

#[function_component]
pub fn ReleasePicker() -> Html {
    log::trace!("Rendering ReleasePicker");

    let manifest = use_manifest();
    let active_release = use_active_release_tracker();
    let active_release_switcher = use_active_release_switcher();
    let active_continuity = use_active_continuity();

    let active_continuity = active_continuity.active();
    let active_release =
        active_continuity.and_then(|ac| active_release.active(ac.reference_name()));

    let options = manifest
        .opt()
        .as_deref()
        .map(|m| m.all_releases())
        .into_iter()
        .flatten()
        .map(|(release, continuity)| PickerFeed {
            display: release.display_name().to_string(),
            hidden: Some(continuity) != active_continuity,
            new_group: release
                .begins_group()
                .map(|inner| inner.map(|inner| inner.to_string())),
            selected: Some(release.reference_name()) == active_release,
            value: release.reference_name().to_string(),
        })
        .collect::<Vec<OptionSegment>>();

    let active_continuity = active_continuity.map(|c| c.reference_name().to_string());
    let onpick = Callback::from(move |new_release: String| {
        if let Some(active_continuity) = active_continuity.clone() {
            active_release_switcher.switch(active_continuity, new_release)
        }
    });

    html! {
        <Picker {options} {onpick} />
    }
}
