use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        async fn reply(ctx: Context, msg: Message, text: String) {
            if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
                println!("Error sending message: {why:?}");
            }
        }

        match msg.content.as_str() {
            "!ping" | "!pung" => reply(ctx, msg, String::from("Womp Womp!")).await,
            "!help" => reply(ctx, msg, String::from("Heya! I am simplistic moderating discord bot, I was created by mickpurple using RUST and Serenity!!")).await,
            _ => reply(ctx, msg, String::from("")).await,
        };
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
