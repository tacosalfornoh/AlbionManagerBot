use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn run(options: &[ResolvedOption]) -> String {
    println!("{:?}", options);
    if let Some(ResolvedOption {
        value: ResolvedValue::String(player),
        ..
    }) = options.first()
    {
        format!("{}'s user id", player)
    } else {
        "Please provide a valid player ".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("user_id").description("Get a user id").add_option(
        CreateCommandOption::new(CommandOptionType::String, "id", "The user to lookup")
            .required(true),
    )
}

