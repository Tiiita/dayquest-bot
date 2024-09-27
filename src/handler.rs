use serenity::{
    all::{Context, EventHandler, Interaction, Ready},
    async_trait,
};

use crate::commands::{self};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        for ele in ctx.cache.guilds() {
            commands::commands::register_all(&ctx, &ele).await;
        }
        println!("Sucessfully connected to bot!\n");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            commands::commands::handle_cmd(&cmd, &ctx).await;
        }
    }
}
