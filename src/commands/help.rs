use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("info", "h")]
fn help(context: &mut Context, message: &Message) -> CommandResult {
    let response = "Commands:```\nread:\nattach an image with the command, and I will tell what text it has\n\nhelp:\ndisplay this message```";

    let _ = message.channel_id.say(&context.http, response);

    Ok(())
}
