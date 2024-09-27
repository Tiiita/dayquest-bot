use dayquest_bot::{config::{self}, handler};
use serenity::{all::GatewayIntents, Client};

#[tokio::main]
async fn main() {
    println!("Running DayQuest Discord Bot!");
    let config = config::load();
    let intents = GatewayIntents::all();

    Client::builder(&config.token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client")
        .start().await.expect("Failed to start client");
}