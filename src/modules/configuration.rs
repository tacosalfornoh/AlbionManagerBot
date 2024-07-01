use crate::Document;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

pub fn create_guild(guild_name: &str, guild_id: &str, autoRole: bool, autoRoleId: &str) -> Document {
    doc! {
        "guildName": guild_name,
        "guildId": guild_id,
        "autoRole": autoRole,
        "autoRoleId": autoRoleId,
    }
}

