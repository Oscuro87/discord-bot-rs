use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::CacheHttp,
    model::{channel::Message, event::MessageUpdateEvent, id::GuildId},
};
use std::sync::Arc;

use crate::utils::bot_reply::reply_question;
use crate::{plugins::*, datastructs::sanitized_message::SanitizedMessage};

pub struct DefaultHandler;

impl DefaultHandler {
    pub fn new() -> Self {
        DefaultHandler {}
    }
}

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        // Tea time and midnight announcer
        fn_tea_time::tea_time_announcer(Arc::new(ctx.clone())).await;
        weather::task_thunderstorm_sentry(Arc::new(ctx.clone())).await;
    }

    #[allow(unused_variables)]
    async fn message(&self, ctx: Context, msg: Message) {
        let being_mentioned: bool = msg.mentions_me(&ctx.clone().http()).await.unwrap_or(false);

        fn_message_announcer::message_announcer(Arc::new(ctx.clone()), msg.clone()).await;

        if being_mentioned {
            let sani: SanitizedMessage = msg.clone().into();
            let question: String = sani.args_single_line;
            let reply: String = reply_question(question);
            let _ = msg.reply(&ctx.clone().http(), reply).await;
        }
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        crate::plugins::edits_watcher::watch_edits(
            &ctx,
            old_if_available.clone(),
            new.clone(),
            event.clone(),
        ).await;
    }
}
