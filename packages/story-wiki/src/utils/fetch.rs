use reqwasm::http::{Request, Response};
use story_wiki_core::manifest::Manifest;
use web_sys::{window, Url};

pub async fn fetch_manifest() -> Option<Manifest> {
    let text = fetch_text("/manifest.toml").await?;
    let out = toml::from_str(&text);
    log::trace!("{out:#?}");
    out.ok()
}

pub async fn fetch(fetch_path: &str) -> Option<Response> {
    let base = window()?.document()?.base_uri().ok().flatten()?;
    let uri = Url::new_with_base(fetch_path, &base).ok()?;
    Request::get(&uri.href()).send().await.ok()
}

pub async fn fetch_binary(target: &str) -> Option<Vec<u8>> {
    fetch(target).await?.binary().await.ok()
}

pub async fn fetch_text(target: &str) -> Option<String> {
    fetch(target).await?.text().await.ok()
}
