use rand::prelude::IteratorRandom;
use regex::Regex;
use serenity::{
    client::Context,
    framework::standard::{
        macros::command,
        Args, CommandResult,
    },
    model::channel::Message,
};

use crate::utils::Roller;
use crate::utils::bot_reply::reply_question;
use crate::datastructs::sanitized_message::SanitizedMessage;

#[command]
#[min_args(1)]
#[aliases("8ball")]
#[description(
    "Ask a question to Anna, she will reply truthfully. Repeated question might (will) annoy her."
)]
#[usage("!8ball [your question]")]
pub async fn eight_ball(ctx: &Context, msg: &Message) -> CommandResult {
    let san: SanitizedMessage = SanitizedMessage::from(msg);
    let question = san.args_single_line.clone();
    let reply = reply_question(question);
    msg.reply(ctx, format!("{}", &reply)).await?;
    Ok(())
}

// TODO: divide long rolls into multiple messages
#[command]
#[min_args(1)]
#[allow(unused_assignments)]
pub async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut dices: u32 = 1;
    let mut faces: u32 = 6;
    let mut modifier: i32 = 0;
    let mut roll_params: String = args.message().to_string();
    roll_params = roll_params.replace::<&str>(" ", "");

    // Regex
    let re = Regex::new(r"(?P<dices>\d*)(?:d|D)(?P<faces>\d+)(?P<mod>-?\+?\d+)?").unwrap();
    let caps = re.captures(roll_params.as_str()).unwrap();

    let dices_text = caps.name("dices").map_or("1", |x| x.as_str());
    let faces_text = caps.name("faces").map_or("6", |x| x.as_str());
    let modifier_text = caps.name("mod").map_or("0", |x| x.as_str());

    dices = u32::from_str_radix(dices_text, 10).unwrap_or(1);
    faces = u32::from_str_radix(faces_text, 10).unwrap_or(6);
    modifier = i32::from_str_radix(modifier_text, 10).unwrap_or(0);

    let results = Roller::roll_mod(dices, faces, modifier);
    msg.reply(ctx, format!("You rolled: {}", results.to_string()))
        .await?;
    Ok(())
}

#[command]
#[min_args(1)]
pub async fn pick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let separator: &str = "|";
    let content = args.message().to_string();
    let split = content.split(separator);

    let pick = split.choose(&mut rand::thread_rng()).expect("Cannot pick any option in picker!");
    let _ = msg.reply(&ctx.http, format!("{}", pick)).await;

    Ok(())
}