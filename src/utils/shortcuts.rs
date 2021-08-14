#![allow(dead_code)]

use serenity::{model::id::ChannelId, prelude::*, utils::MessageBuilder};
use serenity::http::CacheHttp;

pub async fn send(ctx: &Context, target_channel: ChannelId, reply: &mut MessageBuilder) -> () {
    send_or_forward_err(ctx, target_channel, reply).await;
}

pub async fn send_or_console_err(ctx: &Context, target_channel: ChannelId, reply: &mut MessageBuilder) -> () {
    if let Some(err) = send_or_forward_err(ctx, target_channel, reply).await {
        println!("Error: {}", err);
    }
}

pub async fn send_or_discord_err(ctx: &Context, target_channel: ChannelId, error_target_channel: ChannelId, reply: &mut MessageBuilder) -> () {
    if let Some(err) = send_or_forward_err(ctx, target_channel, reply).await {
        let error_message: String = format!("Erreur: {}", err);
        let _ = error_target_channel.say(ctx.http(), error_message).await;
    }
}

pub async fn send_or_forward_err(
    ctx: &Context,
    target_channel: ChannelId,
    reply: &mut MessageBuilder
) -> Option<SerenityError> {
    if let Err(err) = target_channel.say(ctx.http(), reply.build()).await {
        return Some(err);
    }

    None
}
