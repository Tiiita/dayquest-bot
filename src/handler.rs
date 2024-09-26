use serenity::{all::{Context, EventHandler, Ready}, async_trait};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("Bot started sucessfully!\n");
    }
}