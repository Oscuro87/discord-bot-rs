use chrono::offset::Utc;
use chrono::DateTime;
use serenity::model::id::{ChannelId, MessageId, RoleId};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::MessageBuilder,
};
use std::env::current_exe;
use std::fs;

use crate::utils::shortcuts::send_or_discord_err;
use crate::{constants::channels::ERRORS, datastructs::SanitizedMessage};
use crate::constants::channels::INFRARED;
use crate::utils::apis::igdb::query_game_by_name;
use crate::utils::igdb::IGDBGameSearchResponseData;

#[command]
pub async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    let exe = current_exe().unwrap();
    let metas = fs::metadata(exe).unwrap();
    let build_date: DateTime<Utc> = metas.created().unwrap().into();
    let build_tz = build_date + chrono::Duration::hours(2);

    msg.reply(
        ctx,
        format!(
            "\nDiscord bot version {}\nBuilt on {}",
            env!("CARGO_PKG_VERSION"),
            build_tz
        ),
    )
    .await?;

    Ok(())
}

#[command]
#[aliases("move", movemsg)]
#[owners_only]
#[min_args(2)]
#[max_args(2)]
pub async fn move_message_manually(ctx: &Context, msg: &Message) -> CommandResult {
    // Need: Message id, target channel id
    let san: SanitizedMessage = msg.into();
    let src_channel_id: ChannelId = msg.channel_id;
    let msg_id_parsed: u64 = san.arguments.get(0).unwrap().parse::<u64>().unwrap();
    let message_id: MessageId = MessageId(msg_id_parsed);
    let message = ctx
        .http
        .get_message(src_channel_id.into(), message_id.into())
        .await
        .unwrap();
    let chan_id_parsed: u64 = san.arguments.get(1).unwrap().parse::<u64>().unwrap();
    let target_channel_id: ChannelId = ChannelId(chan_id_parsed);
    let original_poster_name: String = message.author.name.clone();

    // Check if source and target channels are diff
    if msg.channel_id == target_channel_id {
        return Ok(());
    }

    // Copy content
    let mut msg_builder: MessageBuilder = MessageBuilder::new();
    let content: String = message.content.clone();
    msg_builder.push_line(content);
    msg_builder.push_line(format!("(Original Poster: {})", original_poster_name));

    // Delete
    let del_result = message.delete(&ctx.http).await;

    if del_result.is_ok() {
        // Send to new channel
        let _ = send_or_discord_err(ctx, target_channel_id, ERRORS.into(), &mut msg_builder).await;
    }

    Ok(())
}

#[command]
#[aliases("notabot")]
pub async fn not_a_bot(ctx: &Context, msg: &Message) -> CommandResult {
    let infrared_role_id = RoleId::from(INFRARED);
    // let everyone_role_id = RoleId::from(EVERYONE);
    let guild_id = msg.guild_id.unwrap();
    let user = msg.author.clone();

    if let Ok(is_infrared) = user.has_role(&ctx.http, guild_id, infrared_role_id).await {
        if is_infrared {
            let _ = msg.reply_mention(&ctx.http, "You're already a confirmed member, congratulations.").await;
        } else {
            if let Err(role_error) = &ctx.http.add_member_role(guild_id.into(), user.id.into(), infrared_role_id.into()).await {
                eprintln!("Cannot assign role to user: {}", role_error.to_string());
            }
            let _ = msg.reply_mention(&ctx.http, "You are now confirmed.").await;
        }
    }

    Ok(())
}

#[command]
pub async fn search(ctx: &Context, msg: &Message) -> CommandResult {
    let sani: SanitizedMessage = msg.into();
    let game_name: String = sani.args_single_line;
    let response: Result<IGDBGameSearchResponseData, reqwest::Error> =
        query_game_by_name(game_name).await;

    if response.is_ok() {
        let res_data: IGDBGameSearchResponseData = response.unwrap();
        let _ = msg.reply_mention(&ctx.http, res_data.to_string()).await;
    } else {
        eprintln!("There was an issue searching for an IGDB game: {}", response.unwrap_err());
    }

    Ok(())
}
