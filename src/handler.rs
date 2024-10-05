use serenity::{
    all::{Context, EventHandler, Interaction, Ready},
    async_trait,
};

use crate::{commands::{self}, ticket};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        for ele in ctx.cache.guilds() {
            commands::commands::register_all(&ctx, &ele).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(cmd) => commands::commands::handle_cmd(&cmd, &ctx).await,
            Interaction::Component(component) => {
                let id = component.data.custom_id.as_str();
                match id {
                    "select_ticket_type" => {
                        ticket::hande_ticket_selection(ctx, component).await;
                    }
                    _ => {},
                }
            }
            _ => {},
        }
    }
}
