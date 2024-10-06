use serenity::{
    all::{
        CommandInteraction, Context, CreateCommand, CreateEmbed,
        CreateInteractionResponseMessage, Permissions,
    },
    Error,
};

use crate::{commands, config::DISCORD_BACKGROUND_COLOR, ticket};

pub const NAME: &str = "send-ticket-panel";
pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let embed: CreateEmbed = CreateEmbed::new()
    .title("Erstelle ein Ticket")
    .color(DISCORD_BACKGROUND_COLOR)
    .description("Wähle eine Ticket Kategorie aus, um ein Ticket zu erstellen, sodass du Kontakt zum Staff aufnimmst!");

    commands::respond(
        ctx,
        interaction,
        CreateInteractionResponseMessage::new()
            .add_embed(embed)
            .components(vec![ticket::get_ticket_selection_menu()]),
    ).await;

    println!("Sent ticket panel in channel: {}", interaction.channel_id);
    Ok(())
}

pub async fn register() -> CreateCommand {
    CreateCommand::new(NAME)
        .description("Sends the ticket panel")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
