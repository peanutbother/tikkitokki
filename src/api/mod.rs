use anyhow::{anyhow, Result};
use async_recursion::async_recursion;
use cached::proc_macro::cached;
use reqwest::IntoUrl;
use serde::Deserialize;
use url::Url;

mod errors;
pub use errors::ApiError;

const API_URL: &str = "https://api16-normal-c-useast1a.tiktokv.com/aweme/v1/feed/?aweme_id=";

#[derive(Debug, Deserialize)]
struct UrlList {
    pub url_list: Vec<String>,
}
#[derive(Debug, Deserialize)]
struct PlayAddress {
    pub play_addr: UrlList,
}
#[derive(Debug, Deserialize)]
struct Video {
    pub video: PlayAddress,
}
#[derive(Debug, Deserialize)]
struct TikTokResponse {
    pub aweme_list: Vec<Video>,
}

#[cached(time = 10, sync_writes = true, result = true)]
pub async fn request_url<'a>(id: String) -> Result<String, ApiError> {
    let response = reqwest::get(format!("{API_URL}{}", id)).await?;
    let TikTokResponse { aweme_list } = response.json().await?;
    let Video {
        video: PlayAddress {
            play_addr: UrlList { url_list },
        },
    } = &aweme_list[0];

    let mut url: Url = url_list[0]
        .clone()
        .parse()
        .map_err(|e| ApiError::InvalidUrl(anyhow!("Failed to parse url: {e}")))?;
    url.set_query(None);

    Ok(url.to_string())
}

#[cached(time = 5, sync_writes = true, result = true)]
pub async fn get_id(url: String) -> Result<String, ApiError> {
    log::debug!("url not yet cached: {url}");
    get_id_body(url).await
}

#[async_recursion]
async fn get_id_body(url: String) -> Result<String, ApiError> {
    log::debug!("parsing {url}");
    let url =
        Url::parse(&url).map_err(|e| ApiError::InvalidUrl(anyhow!("Failed to parse url: {e}")))?;

    log::debug!("parsed url: {url:?}");
    if url.path().contains("/video/") {
        if let Some(segments) = url.path_segments().map(|c| c.collect::<Vec<_>>()) {
            return segments
                .iter()
                .last()
                .cloned()
                .map(String::from)
                .ok_or(ApiError::InvalidUrl(anyhow!("Invalid Url specified")));
        }
    } else if url.domain() == Some("vm.tiktok.com") || url.domain() == Some("vt.tiktok.com") {
        log::debug!("valid short url detected");
        let url = fetch_long_url(url.clone()).await?;
        log::debug!("fetched new url: {url}");
        return get_id_body(url.to_string()).await;
    }
    Err(ApiError::InvalidUrl(anyhow!("Url must contain /video/")))
}

pub async fn fetch_long_url<U: IntoUrl>(url: U) -> Result<String, ApiError> {
    fetch_long_url_body(url.as_str().to_owned()).await
}

#[cached(time = 5, sync_writes = true, result = true)]
async fn fetch_long_url_body(url: String) -> Result<String, ApiError> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::default())
        .build()
        .unwrap();

    log::debug!("fetching long url for {}", url.as_str());
    let request = client.get(url).build().unwrap();
    let url = client.execute(request).await?.url().clone();

    Ok(url.to_string())
}
