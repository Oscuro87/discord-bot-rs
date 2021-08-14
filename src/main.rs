use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use serenity::{client::Client, framework::standard::macros::group};
use std::collections::{hash_map::RandomState, HashSet};

mod buckets;
mod handlers;
mod utils;
mod plugins;
mod constants;
mod datastructs;

use buckets::*;
use handlers::*;
use datastructs::*;
#[allow(unused_imports)]
use plugins::*;

#[group]
#[commands(ping, links, weather)]
pub struct Helpers;

#[group]
#[commands(eight_ball, roll, pick)]
pub struct Fun;

#[group]
#[commands(version)]
pub struct Utilities;

#[group]
pub struct Admin;

#[tokio::main]
async fn main() {
    let infos: bot_info::BotInfo = bot_info::read_bot_infos();
    let framework = StandardFramework::new().configure(|c| {
        let mut owners_hs: HashSet<UserId, RandomState> = HashSet::new();

        for owner_id in infos.owners_ids.iter() {
            let user_id: UserId = UserId(owner_id.clone());
            owners_hs.insert(user_id);
        }

        c.prefix(infos.prefix.clone().as_str());
        c.ignore_bots(infos.ignore_bots);
        c.owners(owners_hs);
        c
    })
    .group(&HELPERS_GROUP)
    .group(&FUN_GROUP)
    .group(&UTILITIES_GROUP)
    .group(&ADMIN_GROUP);

    let handler: DefaultHandler = DefaultHandler::new();

    let mut client = Client::builder(&infos.token)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
