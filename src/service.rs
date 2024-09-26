use std::process;

use serenity::{all::GatewayIntents, Client};

use crate::{config::Config, handler};

pub async fn login(config: Config) -> Client {
    println!("Login in to bot..");

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&config.token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
        process::exit(0);
    }

    println!("Logged in!");
    client
}
