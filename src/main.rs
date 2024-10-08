use colored::Colorize;
use dayquest_bot::{config::{self}, handler};
use serenity::{all::GatewayIntents, Client};

#[tokio::main]
async fn main() {
    println!("{}", "Running DayQuest Discord Bot!".green());
    let config = config::load();
    let intents = GatewayIntents::all();

    Client::builder(&config.token, intents)
        .event_handler(handler::Handler { config })
        .await
        .expect("Err creating client")
        .start().await.expect("Failed to start client");
}