use yew::prelude::*;

use super::OptionSegment;
use crate::components::picker::{Picker, PickerFeed};
use crate::hooks::continuity_switcher::{use_active_continuity, use_continuity_switcher};
use crate::states::manifest::{use_manifest, Manifest};

#[function_component]
pub fn ContinuityPicker() -> Html {
    log::trace!("Rendering ContinuityPicker");

    let manifest = use_manifest();
    let continuity_switcher = use_continuity_switcher();
    let active_continuity = use_active_continuity();

    let continuity_switcher = continuity_switcher.unwrap();
    let manifest = manifest.unwrap();
    let active_continuity = active_continuity.active();

    let options = manifest
        .continuities()
        .into_iter()
        .map(|continuity| PickerFeed {
            display: continuity.display_name().to_string(),
            hidden: false,
            new_group: None,
            selected: Some(continuity) == active_continuity,
            value: continuity.reference_name().to_string(),
        })
        .collect::<Vec<OptionSegment>>();

    let onpick = Callback::from(move |continuity_reference: String| {
        if let Some(new_continuity) = manifest.continuity(&continuity_reference) {
            if let Err(e) = continuity_switcher.switch(new_continuity) {
                log::error!("Coninuity switcher error: {e}");
            }
        } else {
            log::error!("Unknown continuity: {continuity_reference}");
        }
    });

    html! {
        <Picker {options} {onpick} />
    }
}
