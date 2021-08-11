use serenity::{client::Context, framework::standard::{CommandResult, macros::command}, http::CacheHttp, model::{channel::Message, id::ChannelId}};
use std::fs;
use std::env::current_exe;
use chrono::offset::Utc;
use chrono::DateTime;

use crate::utils::SanitizedMessage;
use crate::STICKY;


#[command]
pub async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    let exe = current_exe().unwrap();
    let metas = fs::metadata(exe).unwrap();
    let build_date: DateTime<Utc> = metas.created().unwrap().into();
    let build_tz = build_date + chrono::Duration::hours(2);

    msg.reply(
        ctx, 
        format!("\nAnna version {}\nBuilt on {}", 
            env!("CARGO_PKG_VERSION"),
            build_tz
        )
    ).await?;
    
    Ok(())
}

#[command]
#[aliases(makesticky, createsticky)]
#[owners_only]
pub async fn sticky(ctx: &Context, message: &Message) -> CommandResult {
    let msg: SanitizedMessage = message.into();

    if msg.args_single_line.trim().len() <= 0 {
        return Err("Vous essayez de créer un sticky vide!".into());
    }

    // Delete the user's order message
    let _ = message.delete(&ctx.http());

    println!("+sticky {}", msg.args_single_line);
    STICKY.lock().unwrap().create_sticky(&msg.args_single_line);
    
    Ok(())
}

#[command]
#[aliases(stickyoff, stickoff)]
#[owners_only]
pub async fn clearsticky(ctx: &Context, message: &Message) -> CommandResult {
    println!("clearsticky");
    STICKY.lock().unwrap().clear_posted_message_id();
    Ok(())
}