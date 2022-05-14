use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{footer::Footer, header::Header, set_title::SetTitle},
    hooks::continuity_switcher::use_active_continuity,
    routes::{switch, Route},
    states::{manifest::use_manifest, title::use_title_switcher},
};

/// The entry point for StoryWiki, intended to be attached to the body of a website.
#[function_component]
pub fn Main() -> Html {
    log::trace!("Rendering Main");
    let manifest = use_manifest();
    html! {
        <BrowserRouter>
            if manifest.is_ready() {
                <SetTitle />
                <Header />
                <MainInner />
                <Footer />
            }
        </BrowserRouter>
    }
}

/// Renders the <main> component of StoryWiki
///
/// # Panics
///
///  - If the manifest is not loaded.
///  - If there is no configured default continuity and the page does not
///    specify a continuity.
#[function_component]
pub fn MainInner() -> Html {
    log::trace!("Rendering MainInner");
    let active_continuity = use_active_continuity();
    let manifest = use_manifest();
    let title_switcher = use_title_switcher();

    let continuity_reference = active_continuity.active().unwrap();
    let manifest = manifest.unwrap();

    title_switcher.site(manifest.title.to_string());

    if let Some(continuity) = manifest.continuity(continuity_reference) {
        title_switcher.continuity(continuity.display_name.to_string());
    }

    html! {
        <Switch<Route> render={Switch::render(switch)} />
    }
}
