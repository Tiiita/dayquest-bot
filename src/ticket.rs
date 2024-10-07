use serenity::all::{
    ButtonStyle, ChannelId, ComponentInteraction, ComponentInteractionDataKind, Context, CreateActionRow, CreateAttachment, CreateButton, CreateChannel, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditMessage, GuildChannel, ReactionType, User, UserId
};

use crate::config::{
    Config, DISCORD_BACKGROUND_COLOR, TICKET_CLOSE_BUTTON, TICKET_CREATION_TYPE_SELECTION,
    TICKET_SELECT_BACKEND, TICKET_SELECT_BETA, TICKET_SELECT_FRONTEND, TICKET_SELECT_MOD,
};

pub async fn handle_ticket_button_interaction(
    ctx: &Context,
    interaction: &ComponentInteraction,
    config: &Config,
) {
    if let ComponentInteractionDataKind::Button = interaction.data.kind {
        let button_id = interaction.data.custom_id.as_str();

        let mut selected_application = None;
        match button_id {
            TICKET_SELECT_BACKEND => selected_application = Some("Backend Developer"),
            TICKET_SELECT_BETA => selected_application = Some("Beta"),
            TICKET_SELECT_FRONTEND => selected_application = Some("Frontend Developer"),
            TICKET_SELECT_MOD => selected_application = Some("Moderator / Supporter"),
            _ => {}
        }

        if let Some(selected) = selected_application {
            if let Err(why) = interaction
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(format!("Ausgewählter Bewerbungstyp: **{selected}**")),
                    ),
                )
                .await
            {
                eprintln!(
                    "Failed to send the selected application type message, Err: {:?}",
                    why
                );
            }

            return;
        }

        let channel_id = &interaction.channel_id.get();

        if !config.open_tickets.contains_key(channel_id)
            || config.open_tickets.get(channel_id) == None {
            eprintln!("Failed to find channel (in config) with id: {channel_id}, closing without transcript");
        } else {
            let creator = *config.open_tickets.get(channel_id).unwrap();
            if let Err(why) = ctx.http.get_user(UserId::new(creator)).await {
                eprintln!("Failed to look up user that created the ticket, Err: {why}");
            }
    
            let transcript = create_transcript(&interaction.channel_id).await;
            let creator = ctx.http.get_user(UserId::new(creator)).await.unwrap();
            if let Err(why) = creator.direct_message(&ctx.http, CreateMessage::new()
                    .content("Dein DayQuest Ticket wurde geschlossen. Hier das Transcript. **Wir empfehlen das zu lesen, da dort noch ungelese Nachrichten vorkommen können**")
                    .add_file(CreateAttachment::bytes(transcript, "dein-transcript.txt")))
                    .await
            {
                eprintln!("Failed to send ticket close msg to user: {}, Err: {:?}", creator.id.get(), why);
            }
        }

        if let Err(why) = interaction.channel_id.delete(&ctx.http).await {
            eprintln!("Failed to delete ticket channel, err: {:?}", why);
            return;
        }

        println!(
            "Closed ticket: {}",
            interaction.channel.clone().unwrap().name.unwrap()
        );
    }
}

pub async fn handle_ticket_selection(
    ctx: &Context,
    interaction: &mut ComponentInteraction,
    config: &Config,
) {
    if interaction.data.custom_id.as_str() != TICKET_CREATION_TYPE_SELECTION {
        return;
    }

    //Reset select menu
    if let Err(why) = interaction
        .message
        .edit(
            &ctx.http,
            EditMessage::new().components(vec![get_ticket_selection_menu()])
        ).await {
        eprintln!("Failed to update ticket creation select menu, Err: {why}");
    }

    if let ComponentInteractionDataKind::StringSelect { ref values } = interaction.data.kind {
        let selected = values.get(0).unwrap().as_str();

        let id = interaction.user.id.get();
        let mut ticket = match selected {
            "application" => Ticket::new(id, TicketType::Application),
            "support" => Ticket::new(id, TicketType::Support),
            "question" => Ticket::new(id, TicketType::Question),
            _ => {
                eprintln!(
                    "{} selected unknown ticket type: {selected}",
                    interaction.user.id
                );

                return;
            }
        };

        ticket.create(&ctx, &selected, &interaction, &config).await;
        interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .content(
                            "Es wurde ein Ticket erstellt: <#".to_owned()
                                + ticket.channel.unwrap().id.get().to_string().as_str()
                                + ">",
                        ),
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
        config: &Config,
    ) {
        let guild = interaction.guild_id.unwrap();
        let channel_name = selected.to_owned() + "-" + interaction.user.name.as_str();
        println!("Created ticket: {channel_name}");

        match guild
            .create_channel(&ctx.http, CreateChannel::new(&channel_name))
            .await
        {
            Ok(channel) => {
                if let Err(err) = channel
                    .send_message(
                        &ctx.http,
                        self.ticket_type
                            .ticket_msg(config, interaction.user.id.get()),
                    )
                    .await
                {
                    eprintln!("Error sending ticket message: {err}")
                }
                self.channel = Some(channel);
            }
            Err(why) => {
                eprintln!("Failed to create ticket channel, err: {:?}", why);
                return;
            }
        }
    }
}

pub async fn create_transcript(channel: &ChannelId) -> String {
    "Unimplemented".into()
}

pub enum TicketType {
    Support,
    Application,
    Question,
}

impl TicketType {
    pub fn ticket_msg(&self, config: &Config, creator_id: u64) -> CreateMessage {
        let mut embed = CreateEmbed::new().color(DISCORD_BACKGROUND_COLOR);
        let msg = CreateMessage::new();
        let close_button = CreateButton::new(TICKET_CLOSE_BUTTON)
            .label("Ticket schließen")
            .style(ButtonStyle::Danger);

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
                        .style(ButtonStyle::Success),
                    CreateButton::new(TICKET_SELECT_FRONTEND)
                        .label("Frontend Dev")
                        .style(ButtonStyle::Success),
                    CreateButton::new(TICKET_SELECT_BACKEND)
                        .label("Backend Dev")
                        .style(ButtonStyle::Success),
                    CreateButton::new(TICKET_SELECT_MOD)
                        .label("Moderator")
                        .style(ButtonStyle::Success),
                ]));
            }

            TicketType::Question => {
                embed = embed
                .title("Frage")
                .description("Nenne deine Frage. Unser Team versucht diese so schnell wie möglich zu beantworten :D!");
            }
        }

        let ticket_role_mention = format!("<@&{}>", config.ticket_role);
        let user_mention = format!("<@{}>", creator_id);
        msg.content(format!("{ticket_role_mention} {user_mention}"))
            .embed(embed)
            .components(components)
    }
}

pub fn get_ticket_selection_menu() -> CreateActionRow {
    CreateActionRow::SelectMenu(
        CreateSelectMenu::new(
            TICKET_CREATION_TYPE_SELECTION,
            CreateSelectMenuKind::String {
                options: vec![
                    CreateSelectMenuOption::new("Support", "support")
                        .emoji(ReactionType::Unicode("🤙".into())),
                    CreateSelectMenuOption::new("Bewerbung", "application")
                        .emoji(ReactionType::Unicode("📕".into())),
                    CreateSelectMenuOption::new("Frage", "question")
                        .emoji(ReactionType::Unicode("📍".into())),
                ],
            },
        )
        .placeholder("Wähle eine Kategorie"),
    )
}
