use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use std::fs;
use std::io::{prelude::*, BufReader};
// use std::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use warp::Filter;

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

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     let (status_line, filename) = match &request_line[..] {
//         "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
//         _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
//     };

//     let contents = fs::read_to_string(filename).unwrap();
//     let length = contents.len();

//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }

#[tokio::main]
async fn main() {
    // loads env
    dotenv().ok();
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
    // if let Err(why) = client.start().await {
    //     println!("Client error: {why:?}");
    // }

    // Spawn the bot in a separate task
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
        tx.send(()).unwrap();
    });

    // start web server
    // let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    // }

    let route = warp::path::end().map(|| warp::reply::html("Hello, world!"));

    let server = warp::serve(route).bind(([0, 0, 0, 0], 8080));

    // Run the web server and bot concurrently
    tokio::select! {
        _ = server => {},
        _ = rx => {},
    }
}
