use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::framework::standard::macros::command;

use crate::utils::stock_utils::{get_stock_price, epoch_to_date};

// Create a serenity-rs command to get the stock price of a given stock.
#[command]
#[description = "Get the stock price of a given stock."]
#[usage = "<stock>"]
#[example = "$AAPL"]
#[aliases("stock", "ticker")]
pub async fn stocks(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut stock_name: String = args.rest().into();

    // If stock_name starts with a "$", remove it.
    if stock_name.starts_with("$") {
        stock_name.remove(0);
    }

    if stock_name.is_empty() {
        msg.channel_id.say(&ctx.http, "Please provide a stock name.").await?;
        return Ok(());
    }

    let stock_price = get_stock_price(stock_name.clone()).await;
    

    // If stock_price is an error, return the error message
    if stock_price.is_err() {
        msg.channel_id.say(&ctx.http, &stock_price.unwrap_err()).await?;
        return Ok(());
    } else {
        // Transform stock_price into a structured string (each field on a new line), with the following format: field: value
        let stock_answer = match stock_price {
            Ok(stock_price) => {
                format!(
                    "Stock info for ${}\nName: {}\nCurrent Trade Price: ${}\nAnalysts Sentiment: {}\nEarning Call Date: {}\nExchange: {}",
                    stock_price.ticker,
                    stock_price.name,
                    stock_price.price,
                    stock_price.rating,
                    epoch_to_date(stock_price.earning_call_date),
                    stock_price.exchange
                )
            }
            Err(error) => error,
        };
        // Send the stock price to the channel
        msg.reply(&ctx.http, &stock_answer).await?;
    }

    Ok(())
}
