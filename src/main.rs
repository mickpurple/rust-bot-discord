use dotenvy::dotenv;
// use serenity::async_trait;
// use serenity::model::channel::Message;
// use serenity::prelude::*;

use poise::serenity_prelude as serenity;
use reqwest;
use serde_json;
use std::env;
use tokio::sync::oneshot;
use warp::Filter;

// struct Handler;

// #[async_trait]
// impl EventHandler for Handler {
//     async fn message(&self, ctx: Context, msg: Message) {
//         async fn reply(ctx: Context, msg: Message, text: String) {
//             if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
//                 println!("Error sending message: {why:?}");
//             }
//         }

//         match msg.content.as_str() {
//             "!ping" | "!pung" => reply(ctx, msg, String::from("Womp Womp!")).await,
//             "!help" => reply(ctx, msg, String::from("Heya! I am simplistic moderating discord bot, I was created by mickpurple using RUST and Serenity!!")).await,
//             _ => reply(ctx, msg, String::from("")).await,
//         };
//     }
// }

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Shows a random meme from Reddit
#[poise::command(slash_command, prefix_command)]
async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    let mut m: String = String::from("meme");

    let res = reqwest::get("https://meme-api.com/gimme/meme").await;

    match res {
        Ok(response) if response.status().is_success() => {
            // Attempt to parse the response JSON
            let data: serde_json::Value = response.json().await?;

            println!("Data: {:?}", data);

            // Safely access the "preview" array and get the last element
            if let Some(preview_array) = data["preview"].as_array() {
                // Ensure the preview array is not empty
                if let Some(last_image) = preview_array.last() {
                    // Make sure the last element is a valid string
                    if let Some(image_url_str) = last_image.as_str() {
                        m = image_url_str.to_string();
                    } else {
                        println!("Error: Last preview element is not a valid string.");
                    }
                } else {
                    println!("Error: 'preview' array is empty.");
                }
            } else {
                println!("Error: 'preview' key is missing or not an array.");
            }

            let response = format!("{}", m);
            ctx.say(response).await?;
        }
        Ok(response) => {
            println!(
                "Error: Received non-success status code: {}",
                response.status()
            );
        }
        Err(err) => {
            println!("Error: Request failed - {}", err);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // loads env
    dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), meme()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Create a new instance of the Client, logging in as a bot.
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Spawn the bot in a separate task
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
        tx.send(()).unwrap();
    });

    let route = warp::path::end().map(|| warp::reply::html("Hello, world!"));
    let server = warp::serve(route).bind(([0, 0, 0, 0], 8080));

    // Run the web server and bot concurrently
    tokio::select! {
        _ = server => {},
        _ = rx => {},
    }
}
