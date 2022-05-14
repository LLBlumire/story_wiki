use yew::prelude::*;

use crate::states::title::use_title_switcher;

#[function_component]
pub fn RouteNotFound() -> Html {
    log::trace!("Rendering RouteNotFound");
    let title = use_title_switcher();
    title.page("Not Found".to_string());
    html! {}
}
