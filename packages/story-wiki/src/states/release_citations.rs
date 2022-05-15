use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Store)]
pub struct State {
    shown: bool,
}

pub struct ReleaseCitationToggler {
    dispatch: Dispatch<State>,
}

impl ReleaseCitationToggler {
    pub fn toggle<E: 'static>(&self) -> Callback<E> {
        self.dispatch.reduce_callback(move |state| {
            log::debug!("Toggling Release Citations");
            web_sys::window()
                .map(|window| window.alert_with_message("Release Citations In Development"));
            state.shown = false;
        })
    }
}

#[hook]
pub fn use_release_citations_toggle() -> ReleaseCitationToggler {
    ReleaseCitationToggler {
        dispatch: Dispatch::new(),
    }
}

#[hook]
pub fn use_release_citations() -> bool {
    use_store_value::<State>().shown
}
