use anyhow::Result;
use getset::Getters;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::client::Workshop;
use crate::interfaces::WorkshopItem;
use crate::STEAM_API_URL;
use crate::USER_AGENT;

const INTERFACE_NAME: &str = "ISteamRemoteStorage";

// WORKSHOP ITEMS:
#[doc(hidden)]
#[derive(Debug, Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
struct WSResponse<T> {
    response: T
}

#[doc(hidden)]
#[derive(Debug, Getters, Serialize, Deserialize, Clone, PartialEq)]
#[getset(get = "pub")]
pub struct WorkshopItemList {
    publishedfiledetails: Vec<WorkshopItem>,
}

// WORKSHOP COLLECTIONS:
#[doc(hidden)]
#[derive(Serialize, Deserialize)]
struct WSCollectionResponseBody {
    result: u8,
    resultcount: u8,
    collectiondetails: Vec<WSCollectionBody>
}
#[doc(hidden)]
#[derive(Serialize, Deserialize)]
struct WSCollectionBody {
    publishedfileid: String,
    result: u8,
    children: Vec<WSCollectionChildren>
}
#[doc(hidden)]
#[derive(Serialize, Deserialize)]
struct WSCollectionChildren {
    publishedfileid: String,
    sortorder: u8,
    filetype: u8
}

/// Fetches the latest WorkshopItem per each addon id
pub fn get_published_file_details(workshop: &Workshop, fileids: &[String]) -> Result<Vec<WorkshopItem>> {
    let endpoint = format!("{STEAM_API_URL}/{INTERFACE_NAME}/GetPublishedFileDetails/v1/");

    let mut params = HashMap::new();
    params.insert("itemcount".to_owned(), fileids.len().to_string());
    fileids.iter().enumerate().for_each(|(index, id)| {
        params.insert(format!("publishedfileids[{index}]"), id.to_string());
    });

    let details: WSResponse<WorkshopItemList> = workshop.client()
        .post(endpoint)
        .header("User-Agent", &USER_AGENT.to_string())
        .form(&params)
        .send()?
        .error_for_status()?
        .json()?;

    Ok(details.response.publishedfiledetails().to_vec())
}

/// Gets the collection details (all the children of this item). Returns a list of children fileids which can be sent directly to get_published_file_details()
pub fn get_collection_details(workshop: &Workshop, fileid: &str) -> Result<Option<Vec<String>>> {
    let endpoint = format!("{STEAM_API_URL}/{INTERFACE_NAME}/GetCollectionDetails/v1/");

    let mut params = HashMap::new();
    params.insert("collectioncount", "1".to_string());
    params.insert("publishedfileids[0]", fileid.to_string());

    let details: WSResponse<WSCollectionResponseBody> = workshop.client()
        .post(endpoint)
        .header("User-Agent", USER_AGENT.to_string())
        .form(&params)
        .send()?
        .error_for_status()?
        .json()?;

    if details.response.resultcount > 0 {
        let ids = details.response.collectiondetails[0].children.iter()
            .map(|child| child.publishedfileid.to_owned())
            .collect();

        Ok(Some(ids))
    } else {
        Ok(None)
    }
}
