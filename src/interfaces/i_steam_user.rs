use anyhow::{anyhow, Result};
use getset::Getters;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::client::Workshop;
use crate::STEAM_API_URL;
use crate::USER_AGENT;

const INTERFACE_NAME: &str = "ISteamUser";

// WORKSHOP ITEMS:
#[doc(hidden)]
#[derive(Debug, Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
struct WSResponse<T> {
    response: T
}

#[derive(Debug, Getters, Serialize, Deserialize, Clone)]
#[getset(get = "pub")]
pub struct PlayerList {
    players: Vec<Player>,
}

#[derive(Debug, Getters, Serialize, Deserialize, Clone)]
#[getset(get = "pub")]
pub struct Player {
    steamid: String,
    communityvisibilitystate: u32,
    profilestate: u32,
    personaname: String,
    profileurl: String,
    avatar: String,
    avatarmedium: String,
    avatarfull: String,
    avatarhash: String,
    personastate: u32,
}

pub fn get_player_names(workshop: &Workshop, steam_ids: &[String]) -> Result<HashMap<String, String>> {
    Ok(get_player_summaries(workshop, steam_ids)?.iter().map(|player| (player.steamid.to_owned(), player.personaname.to_owned())).collect())
}

pub fn get_player_summaries(workshop: &Workshop, steam_ids: &[String]) -> Result<Vec<Player>> {
    let endpoint = format!("{STEAM_API_URL}/{INTERFACE_NAME}/GetPlayerSummaries/v2/");
    let api_key = workshop.apikey().clone().ok_or(anyhow!("Missing api key for authed-only method: get_player_summaries"))?;

    // This api has a 100 ids limit per call. We need to split it in multiple calls.
    let mut player_list = vec![];
    for steam_ids in steam_ids.chunks(100) {
        let mut params = HashMap::new();
        params.insert("key".to_owned(), api_key.to_string());
        params.insert("steamids".to_owned(), steam_ids.join(",").to_string());

        let details: WSResponse<PlayerList> = workshop.client()
            .get(&endpoint)
            .header("User-Agent", USER_AGENT.to_string())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .query(&params)
            .send()?
            .error_for_status()?
            .json()?;
        player_list.extend_from_slice(details.response().players());
    }

    Ok(player_list)
}
