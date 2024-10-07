use serenity::{
    all::{Context, EventHandler, Interaction, Ready},
    async_trait,
};

use crate::{
    commands::{self},
    config::Config,
    ticket,
};

pub struct Handler {
    pub config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        for ele in ctx.cache.guilds() {
            commands::register_all(&ctx, &ele).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(cmd) => commands::handle_cmd(&cmd, &ctx).await,
            Interaction::Component(mut component) => {
                ticket::handle_ticket_button_interaction(&ctx, &component, &self.config).await;
                ticket::handle_ticket_selection(&ctx, &mut component, &self.config).await;
            }
            _ => {}
        }
    }
}
