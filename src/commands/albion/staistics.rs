use serde_json::Value;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use ureq::Error;

fn fetch_data(url: String) -> Result<Value, Error> {
    let response = ureq::get(&url).call()?;
    let body = response.into_string()?;
    let json: Value = serde_json::from_str(&body).unwrap();
    Ok(json)
}

fn check_server(server: String) -> String {
    let mut srv = String::new();
    if server.contains("West") {
        srv = String::from("");
    } else if server.contains("Europe") {
        srv = String::from("-ams");
    } else if server.contains("East") {
        srv = String::from("-sgp");
    }
    srv
}

fn guild_api(guild: String, server: String, mut srv: String) -> String {
    srv = check_server(server);
    let url = format!("https://gameinfo{}.albiononline.com/api/gameinfo/search?q={}", srv, guild);
    let json = fetch_data(url.clone()).unwrap();
    let guild = json["guilds"][0]["Name"].as_str().unwrap();
    let guild_id = json["guilds"][0]["Id"].as_str().unwrap();
    let url = format!("https://gameinfo{}.albiononline.com/api/gameinfo/guilds/{}", srv, guild_id);
    let json = fetch_data(url.clone()).unwrap();
    println!("{:?}", url);
    println!("{:?}", json);
    let alliance = json["AllianceName"].as_str().unwrap();
    format!("Guild: {}\nGuildID: {}\nAlliance: {}", guild, guild_id, alliance)
}

fn player_api(player: String, server: String, mut srv: String) -> String {
    srv = check_server(server);
    let url = format!("https://gameinfo{}.albiononline.com/api/gameinfo/search?q={}", srv, player);
    let json = fetch_data(url.clone()).unwrap();
    let player = json["players"][0]["Name"].as_str().unwrap();
    let player_id = json["players"][0]["Id"].as_str().unwrap();
    let url = format!("https://gameinfo{}.albiononline.com/api/gameinfo/players/{}", srv, player_id);
    let json = fetch_data(url.clone()).unwrap();
    println!("{:?}", url);
    println!("{:?}", json);
    let guild = json["GuildName"].as_str().unwrap();
    format!("Player: {}\nPlayerID: {}\nGuild: {}", player, player_id, guild)
}

fn albion_api(player: String, server: String) -> String {
    let srv = check_server(server.clone());
    if player.contains("guild") {
        let guild = player.replace("guild ", "");
        return guild_api(guild, server, srv);
    } else {
        return player_api(player, server, srv);
    }
}

pub fn run(options: &[ResolvedOption<'_>]) -> String {
    // println!("{:?}", options);
    if let Some(ResolvedOption {
                    value: ResolvedValue::SubCommand(sub_command),
                    ..
                }) = options.first()
    {
        if let Some(ResolvedOption {
                        value: ResolvedValue::String(player),
                        ..
                    }) = sub_command.first()
        {
            if let Some(ResolvedOption {
                            value: ResolvedValue::String(server),
                            ..
                        }) = sub_command.last()
            {
                return albion_api(player.to_string(), server.to_string());
            } else {
                "Please provide a valid server".to_string()
            }
        } else {
            "Please provide a valid player".to_string()
        }
    } else {
        "Please provide a valid sub command".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("albion").description("Get Albion Online statistics").add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "user", "The statistics to get").add_sub_option(
            CreateCommandOption::new(CommandOptionType::String, "player", "The player to get statistics").required(true)
        ).add_sub_option(
            CreateCommandOption::new(CommandOptionType::String, "server", "The server to get statistics").add_string_choice("West", "West").add_string_choice("Europe", "Europe").add_string_choice("East", "East").required(true)
        )
    ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "guild", "The statistics to get").add_sub_option(
            CreateCommandOption::new(CommandOptionType::String, "guild", "The guild to get statistics").required(true)
        ).add_sub_option(
            CreateCommandOption::new(CommandOptionType::String, "server", "The server to get statistics").add_string_choice("West", "West").add_string_choice("Europe", "Europe").add_string_choice("East", "East").required(true)
        )
    )
}