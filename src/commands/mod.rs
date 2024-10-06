pub mod send_ticket_panel;

use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, GuildId};

pub async fn register_all(ctx: &Context, guild: &GuildId) {
    if let Err(err) = guild
        .set_commands(
            &ctx.http,
            vec![send_ticket_panel::register().await],
        )
        .await
    {
        eprintln!(
            "Error registering command(s) for guild: {}, Error: {err}",
            guild
        );
    }
}

pub async fn handle_cmd(cmd: &CommandInteraction, ctx: &Context) {
    match cmd.data.name.as_str() {
        send_ticket_panel::NAME => {
            if let Err(err) = send_ticket_panel::run(ctx, cmd).await {
                eprintln!("Executed command '{}' with Err: {err}", send_ticket_panel::NAME);
                return;
            }
        },
        _ => { println!("Received unknown command: {}", cmd.data.name) }
    }
}

pub async fn respond(ctx: &Context, cmd: &CommandInteraction, response: CreateInteractionResponseMessage) {
    let builder = CreateInteractionResponse::Message(response);
    if let Err(why) = cmd.create_response(&ctx.http, builder).await {
        println!("Failed to respond to slash command: {why}");
    }
}
