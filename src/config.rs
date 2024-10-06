use std::collections::HashMap;

use figment::{providers::{Format, Json}, Figment};
use serde::Deserialize;
use serenity::all::Color;


pub const DISCORD_BACKGROUND_COLOR: Color = Color::from_rgb(43, 45, 49);

pub const TICKET_CLOSE_BUTTON: &str = "close_ticket";
pub const TICKET_SELECT_BETA: &str = "select_beta_application";
pub const TICKET_SELECT_MOD: &str = "select_beta_mod";
pub const TICKET_SELECT_FRONTEND: &str = "select_beta_frontend";
pub const TICKET_SELECT_BACKEND: &str = "select_beta_backend";
pub const TICKET_CREATION_TYPE_SELECTION: &str = "ticket_selection_type";

pub fn load() -> Config {
    let config = Figment::new()
    .merge(Json::file("config.json"));

    let config =  config.extract().expect("Failed to load config..");
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