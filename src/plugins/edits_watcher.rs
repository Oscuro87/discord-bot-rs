use crate::{
    constants::channels::{EDITS, ERRORS},
    utils::shortcuts::send_or_discord_err,
};
use serenity::{
    client::Context,
    model::{channel::Message, event::MessageUpdateEvent},
    utils::MessageBuilder,
};

pub async fn watch_edits(
    ctx: &Context,
    old_msg: Option<Message>,
    new_msg: Option<Message>,
    _event: MessageUpdateEvent,
) -> () {
    let mut relay_msg: MessageBuilder = MessageBuilder::new();

    if old_msg.is_some() {
        let old: Message = old_msg.unwrap();
        relay_msg.push_line(format!("{} edited a message:", old.author.name));
        relay_msg.push_line("Original Message:");
        relay_msg.push_line(old.content);
    }

    if new_msg.is_some() {
        let new: Message = new_msg.unwrap();
        relay_msg.push_line("New Message:");
        relay_msg.push_line(new.content);
    }

    send_or_discord_err(&ctx.clone(), EDITS.into(), ERRORS.into(), &mut relay_msg).await;
}
