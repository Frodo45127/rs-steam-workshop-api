use anyhow::{anyhow, Result};
use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::client::Workshop;
use crate::interfaces::WorkshopItem;
use crate::STEAM_API_URL;
use crate::USER_AGENT;

const INTERFACE_NAME: &str = "IPublishedFileService";

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
struct WSSearchIdBody {
    result: u8,
    publishedfileid: String,
}

// SEARCH ITEMs
#[doc(hidden)]
#[derive(Serialize, Deserialize)]
struct WSSearchResponse<T> {
    response: Option<T>,
    total: u8
}

#[doc(hidden)]
#[derive(Debug, Getters, Serialize, Deserialize, Clone, PartialEq)]
#[getset(get = "pub")]
pub struct WorkshopItemList {
    publishedfiledetails: Vec<WorkshopItem>,
}

///Search for workshop items, returns only fileids
pub fn search_ids(client: &Workshop, appid: u64, query: &str, count: usize, use_proxy: bool) -> Result<Option<Vec<String>>> {
    let api_key = client.apikey().clone().ok_or(anyhow!("Missing api key for authed-only method: search_ids"))?;
    let query = &[
        ("page", "1"),
        ("numperpage", &count.to_string()),
        ("search_text", query),
        ("appid", &appid.to_string()),
        ("key", &api_key),
    ];

    let details: WSSearchResponse<WorkshopItemList> = serde_json::from_str(&query_files(client, query, use_proxy)?)?;

    match details.response {
        Some(response) => Ok(Some(response.publishedfiledetails().iter().map(|item| item.publishedfileid().to_owned()).collect())),
        None => Ok(None),
    }
}

///Searches for workshop items, returns full metadata
pub fn search_full(client: &Workshop, appid: u64, query: &str, count: usize, use_proxy: bool) -> Result<Vec<WorkshopItem>> {
    let api_key = client.apikey().clone().ok_or(anyhow!("Missing api key for authed-only method: search_full"))?;
    let query = &[
        ("page", "1"),
        ("numperpage", &count.to_string()),
        ("search_text", query),
        ("appid", &appid.to_string()),
        ("return_metadata", "1"),
        ("key", &api_key),
    ];

    let details: WSSearchResponse<WorkshopItemList> = serde_json::from_str(&query_files(client, query, use_proxy)?)?;

    if details.total > 0 {
        Ok(details.response.unwrap().publishedfiledetails)
    } else {
        Ok(vec!())
    }
}

///Searches for workshop items, returns full metadata
pub fn query_files(client: &Workshop, query: &[(&str, &str)], use_proxy: bool) -> Result<String> {
    let endpoint = if use_proxy {
        client.proxy_url().clone().ok_or_else(|| anyhow!("Trying to use a proxy but no proxy url has been found."))?
    } else {
        format!("{STEAM_API_URL}/{INTERFACE_NAME}/QueryFiles/v1/")
    };
    client.client()
        .get(endpoint)
        .header("User-Agent", USER_AGENT.to_string())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .query(query)
        .send()?
        .text()
        .map_err(From::from)
}
