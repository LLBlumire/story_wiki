use yew::prelude::*;

use super::OptionSegment;
use crate::components::picker::{Picker, PickerFeed};
use crate::hooks::continuity_switcher::{use_continuity_switcher, ContinuitySwithcerHandle};
use crate::states::manifest::{use_manifest, Continuity, Manifest};

#[function_component]
pub fn ContinuityPicker() -> Html {
    log::trace!("Rendering ContinuityPicker");

    let manifest = use_manifest();
    let continuity = use_continuity_switcher();

    let active_continuity = continuity
        .as_ref()
        .and_then(ContinuitySwithcerHandle::active);

    let options = manifest
        .opt()
        .as_deref()
        .map(Manifest::continuities)
        .into_iter()
        .flatten()
        .map(
            |Continuity {
                 display_name,
                 reference_name,
             }| PickerFeed {
                display: display_name.clone(),
                hidden: false,
                new_group: None,
                selected: Some(reference_name.as_str()) == active_continuity,
                value: reference_name.clone(),
            },
        )
        .collect::<Vec<OptionSegment>>();

    let onpick = Callback::from(move |new_continuity| {
        if let Some(continuity) = continuity.clone() {
            if let Err(e) = continuity.switch(new_continuity) {
                log::error!("Coninuity switcher error: {e}");
            }
        }
    });

    html! {
        <Picker {options} {onpick} />
    }
}
