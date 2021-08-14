use std::sync::Arc;

use serenity::{client::Context, framework::standard::{macros::command, CommandResult}, http::CacheHttp, model::channel::Message, utils::MessageBuilder};

use crate::plugins::weather::{fetch_weather_for_city, kelvin_to_celsius};
use crate::datastructs::sanitized_message::SanitizedMessage;

#[command]
#[owners_only]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
pub async fn links(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        format!(
            "
        Twitch:
        - Star and Grey: https://www.twitch.tv/star_and_grey
        
        Youtube:
        - Grey Monster: https://www.youtube.com/channel/UCFsWs9C4oDm_JMtmpLFX7eQ
        - Emka: https://www.youtube.com/channel/UChUWneEkjNMqLNpp-vQ2DRQ"
        ),
    )
    .await?;
    Ok(())
}

#[command]
#[aliases(meteo, météo)]
pub async fn weather(ctx: &Context, msg: &Message) -> CommandResult {
    let san_msg = SanitizedMessage::from(msg);
    let delete_reply_after_secs: u64 = 60 * 2;
    let mut city: String = "".into();

    if san_msg.are_any_arguments_present() {
        // City specified by user, pass it to the weather fetcher function
        city = san_msg.args_single_line;
    }

    match fetch_weather_for_city(city).await {
        Ok(weather) => {
            if weather.weather.len() > 0 {
                let msg_builder = MessageBuilder::new()
                    .user(msg.author.id)
                    .push("\nMétéo à ")
                    .push_bold_line(weather.name)
                    .push_line(format!("Ciel: {}.", weather.weather[0].description))
                    .push_line(format!(
                        "Il fait {:.1}°C ({:.1}°C ressenti).",
                        kelvin_to_celsius(weather.main.temp),
                        kelvin_to_celsius(weather.main.feels_like)
                    ))
                    .push_line(format!("Humidité {}%.", weather.main.humidity))
                    .build();
                if let Ok(sent) = msg.channel_id.say(&ctx, msg_builder).await {
                    let ctx_a = Arc::new(ctx.clone());
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(delete_reply_after_secs)).await;
                        let _ = sent.delete(&ctx_a.http()).await;
                    });
                }
            }
        }
        Err(err) => {
            let msg_builder = MessageBuilder::new()
                .user(msg.author.id)
                .push_line(format!("Erreur: {}", err))
                .build();
            let _ = msg.channel_id.say(&ctx, msg_builder).await;
        }
    }

    Ok(())
}
