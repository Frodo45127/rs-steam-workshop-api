use getset::*;
use reqwest::blocking::Client;

#[derive(Debug, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Workshop {
    apikey: Option<String>,
    client: Client,
    proxy_url: Option<String>
}

impl Workshop {

    ///Creates a new workshop instance, client will be auto created if None
    pub fn new(client: Option<Client>) -> Self {
        let client = match client {
            Some(client) => client,
            None => reqwest::blocking::Client::new()
        };
        Self {
            apikey: None,
            client,
            proxy_url: None
        }
    }
}
