use serenity::async_trait;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};


pub fn run(_options: &[ResolvedOption]) -> String {
    "Pong!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Replies with Pong!")
}





