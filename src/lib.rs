//! # steam_workshop_api
//!
//! This library provides access to the steam web apis. Uses reqwest::blocking under the hood.
//!
//! # Getting Started
//!
//! To access any web api that requires no authentication you need to create a new instance of Workshop,
//! then pass it to the api you want to call:
//! ```rust
//! use steam_workshop_api::client::Workshop;
//! use steam_workshop_api::interfaces::i_steam_remote_storage;
//! 
//! //Either pass in a Some(reqwest::blocking::Client) or leave None for it to be autocreated
//! let wsclient = Workshop::new(None);
//! i_steam_remote_storage::get_published_file_details(&wsclient, &[123456]);
//! ```
//! 
//! # Using Authorized Methods 
//! 
//! Authorized Method can be called with an instance of Workshop that has an API Key set:
//! ```rust
//! use steam_workshop_api::client::Workshop;
//! use steam_workshop_api::interfaces::i_published_file_service;
//! 
//! let mut wsclient = Workshop::new(None);
//! wsclient.set_apikey(Some("MY_API_KEY".to_owned()));
//! i_published_file_service::search_ids(...);
//! ```
//!
//! # Using Proxied Methods 
//! 
//! Proxied methods are identical to normal methods, except can use a third party server to proxy (and keep the appkey private).
//! Methods that support proxy have a "use_proxy" param.
//!
//! ```rust
//! use steam_workshop_api::client::Workshop;
//! use steam_workshop_api::interfaces::i_published_file_service;
//!
//! let mut wsclient = Workshop::new(None);
//! wsclient.set_proxy_url(Some("https://jackz.me/l4d2/scripts/search_public.php".to_owned()));
//! i_published_file_service::search_ids(...);
//! ```

use lazy_static::lazy_static;

pub mod client;
pub mod interfaces;

lazy_static! {
    static ref USER_AGENT: String = format!("{}/v{}", "rs-steamwebapi", env!("CARGO_PKG_VERSION"));
}

const STEAM_API_URL: &str = "https://api.steampowered.com";

