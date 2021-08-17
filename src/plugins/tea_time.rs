#![allow(dead_code)]

use chrono::{Timelike, Utc};
use rand::{prelude::SliceRandom, thread_rng};
use serenity::{client::Context, model::id::ChannelId};
use std::ops::Add;
use std::sync::Arc;
use super::weather::{fetch_weather_default_city, kelvin_to_celsius};

use crate::constants::channels::ZIGGURAT;

pub async fn tea_time_announcer(ctx: Arc<Context>) -> () {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(60000)).await;
            let utc_plus_2 = Utc::now().add(chrono::Duration::hours(2));
            
            // Get the weather to decide for hot or cold beverage.
            let cur_weather = fetch_weather_default_city().await;
            let mut beverage: String = "tea".into();
            if cur_weather.is_ok() {
                let temperature_kelvins: f32 = cur_weather.unwrap().main.temp;
                let temperature_celsius: f32 = kelvin_to_celsius(temperature_kelvins);

                beverage = match temperature_celsius {
                    temp if temp <= 0.0 => "hot lava tea or chocolate".into(),
                    temp if temp > 0.0 && temp < 21.0 => "tea".into(),
                    temp if temp >= 21.0 && temp <= 30.0 => "iced tea".into(),
                    temp if temp > 30.0 => "super frozen tea".into(),
                    _ => "tea".into(),
                }
            }
            
            // Tea time 16h and 22h
            if (utc_plus_2.hour() == 16 && utc_plus_2.minute() < 1)
                || (utc_plus_2.hour() == 22 && utc_plus_2.minute() < 1)
            {
                if let Err(why) = ChannelId(ZIGGURAT)
                    .send_message(&ctx, |m| {
                        m.content(format!("It's {} time!", beverage));
                        m.allowed_mentions(|am| am.parse(serenity::builder::ParseValue::Users));
                        m
                    })
                    .await
                {
                    eprintln!("{}", why);
                }
            }

            // Midnight
            if utc_plus_2.hour() == 0 && utc_plus_2.minute() < 1 {
                let pick: Vec<&str> = vec![
                    "IL EST MINUIIIIIIIIT ET TOUUUUUUUUUUT VA BIEEEEEEEEEEEN",
                    "ON EST AUJOURD'HUI",
                ];
                let picked: &str = pick.choose(&mut thread_rng()).expect("oops").clone();
                if let Err(why) = ChannelId(ZIGGURAT)
                    .send_message(&ctx, |m| m.content(picked))
                    .await
                {
                    eprintln!("{}", why);
                }
            }
        }
    });
}