use serenity::{async_trait, client::{Context, EventHandler}, http::CacheHttp, model::{channel::{Channel, Message}, id::{ChannelId, GuildId, MessageId}}, utils::MessageBuilder};
use std::sync::Arc;

use crate::{constants::channels, plugins::*, utils::SanitizedMessage};
use crate::utils::bot_reply::reply_question;
use crate::STICKY;

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
        let sani: SanitizedMessage = msg.clone().into();
        fn_message_announcer::message_announcer(Arc::new(ctx.clone()), msg.clone()).await;

        // TODO: Put this in a plugin function
        if being_mentioned {
            let question: String = sani.args_single_line;
            let reply: String = reply_question(question);
            let _ = msg.reply(&ctx.clone().http(), reply).await;
        }

        // Sticky message rewrite at bottom
        // Does not update nor clear the sticky, see "utilities" bucket for that.
        // TODO: Put this in a plugin function
        if STICKY.lock().unwrap().sticky_exists() {
            let sticky_context = ctx.clone();
            let sticky_message: String = STICKY.lock().unwrap().current_sticky.clone();
            let target_channel_id: ChannelId = ChannelId(channels::ZIGGURAT);

            if STICKY.lock().unwrap().posted_message_id.is_some() {
                let delete_context: Context = ctx.clone();
                let msg_id: MessageId = STICKY.lock().unwrap().posted_message_id.unwrap();
                let target_channel: Channel = target_channel_id.to_channel(&delete_context.http()).await.unwrap();

                if let Channel::Guild(c) = target_channel {
                    // TODO: Write this more cleanly...
                    // TODO: Handle errors...
                    let _ = c.message(&delete_context.http(), msg_id).await.unwrap().delete(&delete_context.http()).await;
                }
            }

            let mut msg_builder = MessageBuilder::new();
            msg_builder.push_line("@everyone ");
            msg_builder.push_line(sticky_message);
            if let Ok(new_sticky) = target_channel_id.say(&sticky_context, msg_builder.build()).await {
                STICKY.lock().unwrap().posted_message_id = Some(new_sticky.id.clone());
            }
        }
    }
}
