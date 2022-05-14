use yew::prelude::*;

use crate::{
    components::md_render::MdRender, states::pages::use_page,
    utils::downloadable_resource::DownloadableResource,
};

#[derive(PartialEq, Properties)]
pub struct PageRenderProps {
    pub resource_path: String,
    pub continuity: String,
}

#[function_component]
pub fn PageRender(props: &PageRenderProps) -> Html {
    log::trace!("Rendering PageRender {}", props.resource_path);
    let page = use_page(&props.resource_path);
    if let DownloadableResource::Ready(page) = page {
        html! { <MdRender content={String::clone(&page)} continuity={props.continuity.clone()} /> }
    } else {
        html! {}
    }
}
