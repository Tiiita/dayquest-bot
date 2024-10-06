
use serenity::all::{
    ButtonStyle, ComponentInteraction, ComponentInteractionDataKind, Context,
    CreateActionRow, CreateButton, CreateChannel, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GuildChannel,
};

use crate::config::{DISCORD_BACKGROUND_COLOR, TICKET_CLOSE_BUTTON, TICKET_SELECT_BACKEND, TICKET_SELECT_BETA, TICKET_SELECT_FRONTEND, TICKET_SELECT_MOD};

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
    _creator: u64,
    ticket_type: TicketType,
}

impl Ticket {
    pub fn new(creator: u64, ticket_type: TicketType) -> Self {
        Ticket {
            channel: None,
            _creator: creator,
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

        match guild
            .create_channel(&ctx.http, CreateChannel::new(&channel_name))
            .await
        {
            Ok(channel) => {
                if let Err(err) = channel.send_message(&ctx.http, self.ticket_type.ticket_msg()).await {
                    eprintln!("Error sending ticket message: {err}")
                }
                self.channel = Some(channel);
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

impl TicketType {
    pub fn ticket_msg(&self) -> CreateMessage {
        let mut embed = CreateEmbed::new().color(DISCORD_BACKGROUND_COLOR);
        let msg = CreateMessage::new();
        let close_button = CreateButton::new(TICKET_CLOSE_BUTTON).label("Ticket schließen").style(ButtonStyle::Danger);

        let mut components = vec![CreateActionRow::Buttons(vec![close_button])];
        match self {
            TicketType::Support => {
                embed = embed.title("Support Ticket")
                .description("Beschreibe dein Anliegen so detailiert wie möglich. Wir versuchen uns so schnell wie möglich um dein Ticket zu kümmern.");
            }

            TicketType::Application => {
                embed = embed
                .title("Bewerbung")
                .description("Bitte schreibe hier deine (kurz) Bewerbung. Unser Team versucht sich so schnell wie möglich um dein Ticket zu kümmern.
                \n\n**Bitte erledige folgende Aufgaben**
                1. Nenne uns dein Alter
                \n2. Wähle deinen Bewerbungstyp
                \n3. (Falls Dev Bewerbung) Schicke einen GitHub Link");

                components.push(CreateActionRow::Buttons(vec![
                    CreateButton::new(TICKET_SELECT_BETA)
                        .label("Beta")
                        .style(ButtonStyle::Primary),
                    CreateButton::new(TICKET_SELECT_FRONTEND)
                        .label("Frontend Dev")
                        .style(ButtonStyle::Primary),
                    CreateButton::new(TICKET_SELECT_BACKEND)
                        .label("Backend Dev")
                        .style(ButtonStyle::Primary),
                    CreateButton::new(TICKET_SELECT_MOD)
                        .label("Moderator")
                        .style(ButtonStyle::Primary),
                ]));
            }

            TicketType::Question => {
                embed = embed
                .title("Frage")
                .description("Nenne deine Frage. Unser Team versucht diese so schnell wie möglich zu beantworten :D!");
            }
        }

        msg.embed(embed)
        .components(components)
    }
}
