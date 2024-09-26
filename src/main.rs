use dayquest_bot::{config::{self, Config}, handler};
use serenity::{all::GatewayIntents, Client};

#[tokio::main]
async fn main() {
    println!("Running DayQuest Discord Bot!");
    let config = config::load();
    start(create_client(config).await).await;
}


pub async fn create_client(config: Config) -> Client {
    let intents = GatewayIntents::all();

    Client::builder(&config.token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client")
}

pub async fn start(mut client: Client) {
    client.start().await.expect("Failed to start discord bot");
}