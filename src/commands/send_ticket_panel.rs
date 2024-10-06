use serenity::{
    all::{
        CommandInteraction, Context, CreateActionRow, CreateCommand, CreateEmbed,
        CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
        CreateSelectMenuOption, Permissions,
    },
    Error,
};

use crate::{commands, config::{DISCORD_BACKGROUND_COLOR, TICKET_CREATION_TYPE_SELECTION}};

pub const NAME: &str = "send-ticket-panel";
pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let embed: CreateEmbed = CreateEmbed::new()
    .title("Erstelle ein Ticket")
    .color(DISCORD_BACKGROUND_COLOR)
    .description("Wähle eine Ticket Kategorie aus, um ein Ticket zu erstellen, sodass du Kontakt zum Staff aufnimmst!");

    let selection_menu = CreateActionRow::SelectMenu(
        CreateSelectMenu::new(
            TICKET_CREATION_TYPE_SELECTION,
            CreateSelectMenuKind::String {
                options: vec![
                    CreateSelectMenuOption::new("🤙 Support", "support"),
                    CreateSelectMenuOption::new("📕 Bewerbung", "application"),
                    CreateSelectMenuOption::new("📍 Frage", "question"),
                ],
            },
        )
        .placeholder("Wähle eine Kategorie"),
    );

    commands::respond(
        ctx,
        interaction,
        CreateInteractionResponseMessage::new()
            .add_embed(embed)
            .components(vec![selection_menu]),
    ).await;

    println!("Sent ticket panel in channel: {}", interaction.channel_id);
    Ok(())
}

pub async fn register() -> CreateCommand {
    CreateCommand::new(NAME)
        .description("Sends the ticket panel")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
