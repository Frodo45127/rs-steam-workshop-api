use getset::Getters;
use serde::{Deserialize, Serialize};


pub mod i_published_file_service;
pub mod i_steam_remote_storage;


#[derive(Debug, Getters, Serialize, Deserialize, Clone, PartialEq)]
#[getset(get = "pub")]
pub struct WorkshopItem {
    result: i8,
    publishedfileid: String,
    creator: Option<String>,
    creator_app_id: Option<u32>,
    consumer_app_id: Option<u32>,
    filename: Option<String>,
    file_size: Option<u64>,
    file_url: Option<String>,
    hcontent_file: Option<String>,
    preview_url: Option<String>,
    hcontent_preview: Option<String>,
    title: Option<String>,
    description: Option<String>,
    time_created: Option<usize>,
    time_updated: Option<usize>,
    visibility: Option<u32>,
    banned: Option<u32>,
    ban_reason: Option<String>,
    subscriptions: Option<u32>,
    favorited: Option<u32>,
    lifetime_subscriptions: Option<u32>,
    lifetime_favorited: Option<u32>,
    views: Option<u32>,
    tags: Option<Vec<WorkshopItemTag>>
}

#[derive(Debug, Getters, Serialize, Deserialize, Clone, PartialEq)]
#[getset(get = "pub")]
pub struct WorkshopItemTag {
    tag: String
}

