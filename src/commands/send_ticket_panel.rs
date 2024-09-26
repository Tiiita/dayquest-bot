use std::cell::Cell;

use serenity::{all::{ClientError, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EmbedMessageBuilding, Message, MessageBuilder, ModalInteraction, Permissions}, Error};

use super::DISCORD_BACKGROUND_COLOR;
pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let embed = CreateEmbed::new()
    .title("Erstelle ein Ticket")
    .color(DISCORD_BACKGROUND_COLOR)
    .description("Wähle eine Ticket Kategorie aus, um ein Ticket zu erstellen, sodass du Kontakt zum Staff aufnimmst!");

    let selection_menu = CreateSelectMenu::new("select_ticket_type", CreateSelectMenuKind::String 
    { options: vec![CreateSelectMenuOption::new("🤙Support", "support_category"),
                    CreateSelectMenuOption::new("")] 
    
    });
    .placeholder("Wähle eine Ticketkategorie")
    interaction.channel_id.send_message(&ctx.http, )
    Ok(())
}

pub async fn register() -> CreateCommand {
    CreateCommand::new("send-ticket-panel")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
