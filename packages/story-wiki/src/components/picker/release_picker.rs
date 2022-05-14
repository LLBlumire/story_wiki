use yew::prelude::*;
use yew_router::hooks::use_route;

use crate::components::picker::{OptionSegment, Picker, PickerFeed};
use crate::hooks::continuity_switcher::use_active_continuity;
use crate::routes::Route;
use crate::states::active_release::{use_active_release_switcher, use_active_release_tracker};
use crate::states::manifest::{use_manifest, Continuity, Release};

#[function_component]
pub fn ReleasePicker() -> Html {
    log::trace!("Rendering ReleasePicker");

    let manifest = use_manifest();
    let active_release = use_active_release_tracker();
    let active_release_switcher = use_active_release_switcher();
    let active_continuity = use_active_continuity();

    let active_continuity = active_continuity.active();
    let active_release = active_continuity.and_then(|ac| active_release.active(ac));

    let options = manifest
        .opt()
        .as_deref()
        .map(|m| m.all_releases())
        .into_iter()
        .flatten()
        .map(
            |(
                Release {
                    display_name,
                    reference_name,
                    begins_group,
                },
                Continuity {
                    reference_name: continuity_reference_name,
                    ..
                },
            )| PickerFeed {
                display: display_name.clone(),
                hidden: Some(continuity_reference_name.as_str()) != active_continuity,
                new_group: begins_group
                    .clone()
                    .map(|n| Some(n).filter(|n| !n.is_empty())),
                selected: Some(reference_name.as_str()) == active_release,
                value: reference_name.clone(),
            },
        )
        .collect::<Vec<OptionSegment>>();

    let active_continuity = active_continuity.map(str::to_string);
    let onpick = Callback::from(move |new_release: String| {
        if let Some(active_continuity) = active_continuity.clone() {
            active_release_switcher.switch(active_continuity, new_release)
        }
    });

    html! {
        <Picker {options} {onpick} />
    }
}
