use std::collections::HashMap;

use figment::{providers::{Format, Json}, Figment};
use serde::Deserialize;

pub fn load() -> Config {
    let config = Figment::new()
    .merge(Json::file("config.json"));

    let config =  config.extract().expect("Failed to load config..");
    println!("Loaded config");
    config
}

#[derive(Deserialize)]
pub struct Config {
    pub token: String,

    #[serde(rename = "ticketChannel")]
    pub ticket_channel: u64,

    #[serde(rename = "modRole")]
    pub mod_role: u64,

    #[serde(rename = "rolesChannel")]
    pub roles_channel: u64,
    pub roles: HashMap<String, u64>,
}