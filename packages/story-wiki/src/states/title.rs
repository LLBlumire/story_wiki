use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub struct State {
    site_title: Option<String>,
    continuity_title: Option<String>,
    page_title: Option<String>,
}
impl Store for State {
    fn new() -> Self {
        State::default()
    }
    fn changed(&mut self) {
        set_title(self);
    }
}

fn set_title(state: &State) -> Option<()> {
    let site = state
        .site_title
        .clone()
        .unwrap_or_else(|| "A StoryWiki".to_string());
    let continuity = state
        .continuity_title
        .as_ref()
        .map(|title| format!(" - {title}"))
        .unwrap_or_default();
    let page = state
        .page_title
        .as_ref()
        .map(|page| format!(" - {page}"))
        .unwrap_or_default();
    web_sys::window()?
        .document()?
        .set_title(&format!("{site}{continuity}{page}"));
    Some(())
}

pub struct TitleSwitcher {
    dispatch: Dispatch<State>,
}

impl TitleSwitcher {
    pub fn site(&self, new: String) {
        self.dispatch
            .reduce(move |state| state.site_title = Some(new));
    }
    pub fn continuity(&self, new: String) {
        self.dispatch
            .reduce(move |state| state.continuity_title = Some(new));
    }
    pub fn page(&self, new: String) {
        self.dispatch
            .reduce(move |state| state.page_title = Some(new));
    }
}

#[hook]
pub fn use_title_switcher() -> TitleSwitcher {
    TitleSwitcher {
        dispatch: Dispatch::new(),
    }
}
