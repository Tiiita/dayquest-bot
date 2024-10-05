use serenity::all::{
    ComponentInteraction, ComponentInteractionDataKind, Context, CreateAllowedMentions, CreateChannel, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GuildChannel, Message, MessageBuilder
};

use crate::DISCORD_BACKGROUND_COLOR;

pub async fn hande_ticket_selection(ctx: Context, component: ComponentInteraction) {
    if let ComponentInteractionDataKind::StringSelect { ref values } = component.data.kind {
        let selected = values.get(0).unwrap().as_str();

        let id = component.user.id.get();
        match selected {
            "application" => {
                Ticket::new(id, TicketType::Application)
                    .create(&ctx, &selected, &component)
                    .await
            }
            "support" => {
                Ticket::new(id, TicketType::Support)
                    .create(&ctx, &selected, &component)
                    .await
            }
            "question" => {
                Ticket::new(id, TicketType::Question)
                    .create(&ctx, &selected, &component)
                    .await
            }
            _ => {
                eprintln!(
                    "{} selected unknown ticket type: {selected}",
                    component.user.id
                );
            }
        }

        component
            .create_response(
                ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content("Es wurde ein Ticket erstellt."),
                ),
            )
            .await
            .expect("Failed to respond to ticket selection");
    }
}

pub struct Ticket {
    channel: Option<GuildChannel>,
    creator: u64,
    ticket_type: TicketType,
}

impl Ticket {
    pub fn new(creator: u64, ticket_type: TicketType) -> Self {
        Ticket {
            channel: None,
            creator,
            ticket_type,
        }
    }

    pub async fn create(
        &mut self,
        ctx: &Context,
        selected: &str,
        interaction: &ComponentInteraction,
    ) {
        let guild = interaction.guild_id.unwrap();
        let channel_name = selected.to_owned() + "-" + interaction.user.name.as_str();
        println!("Created ticket: {channel_name}");


        let message = match self.ticket_type {
            TicketType::Support => {
                CreateMessage::new()
                .allowed_mentions(CreateAllowedMentions::new()
                    .users(users))
                .add_embed(CreateEmbed::new()
                .color(DISCORD_BACKGROUND_COLOR))
                .content("Bitte beschreibe dein Anliegen hier.")
            },
            TicketType::Application => "test",
            TicketType::Question => "test",
        };
       

        match guild
            .create_channel(&ctx.http, CreateChannel::new(&channel_name))
            .await
        {
            Ok(channel) => {
                self.channel = Some(channel);
                channel.send_message(&ctx.http, message).await.ok();
            }
            Err(_) => {
                eprintln!("Failed to create ticket channel");
                return;
            }
        }
    }

    pub async fn create_transcript() -> String {
        "Unimplemented".into()
    }
}

pub enum TicketType {
    Support,
    Application,
    Question,
}
