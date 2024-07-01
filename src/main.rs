mod commands;
mod modules;

use std::env;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, Document},
    Client,
    Collection,
};

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::id::GuildId;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use commands::misc;
use crate::modules::configuration::create_guild;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            "940226887463100416".parse().expect("Failed to parse guild ID"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                misc::ping::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        // Create a global slash command

        let guild_command = Command::set_global_commands(&ctx.http, vec![
            misc::ping::register(),
            commands::albion::staistics::register(),
            commands::admin::user_id::register(),
        ]).await;

        println!("I created the following global slash command: {guild_command:#?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // Print the data of the command to the console
            // println!("Received command interaction: {command:#?}");
            let content = match command.data.name.as_str() {
                "ping" => Some(misc::ping::run(&command.data.options())),
                "albion" => Some(commands::albion::staistics::run(&command.data.options())),
                "user_id" => Some(commands::admin::user_id::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let test = env::var("MONGODB_URI");
    println!("{:?}", test);
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb+srv://root:qq68T*LLkxXvXCQ@albionguildmanagerdsbot.gh6amky.mongodb.net/?retryWrites=true&w=majority&appName=AlbionGuildManagerDSBot";
    // Create a new client and connect to the server
    let mongo = Client::with_uri_str(uri).await.expect("Failed to connect to the server");
    // Get a handle on the movies collection
    let database = mongo.database("test");
    let my_coll: Collection<Document> = database.collection("configurations");
    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "guildName": "Albion ELITE" }, None).await;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    // Print a variable from the document
    let my_movie = my_movie.unwrap();
    let binding = my_movie.expect("REASON");
    let guild_name = binding.get_str("guildId").unwrap();
    println!("The guild name is: {}", guild_name);
    // inert "Hello" as guildName & guildID "31321231" into the collection
// get modules from configuration.rs
    let document = create_guild("AAAA", "2222", true, "");
    my_coll.insert_one(document, None).await.expect("Failed to insert the document");
    // delete the document
    //let document = doc! { "guildName": "HellAoAA" };
    //my_coll.delete_one(document, None).await.expect("Failed to delete the document");
    println!("Hello, world!");

    // Configure the client with your Discord bot token in the environment.
    let token = "MTIzMTg4OTQ2OTYwMzU3Nzk5MA.G44Nk-.XhffrQCLnRNA6WAPSxKDcPG1bcOLrcyO0ZIjkI";

    // Build our client.
    let mut client = serenity::Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}