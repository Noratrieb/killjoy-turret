use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::hook;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub token: String,
    pub auto_react: Vec<(String, String)>,
}

impl TypeMapKey for ConfigFile {
    type Value = Self;
}

#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    let map = ctx.data.read().await;
    let reactions = map.get::<ConfigFile>().unwrap();

    let lowercase_content = msg.content.to_lowercase();
    for (trigger, reaction) in &reactions.auto_react {
        if lowercase_content.contains(trigger) {
            if let Err(why) = msg.react(&ctx.http, ReactionType::Unicode(reaction.clone())).await {
                eprintln!("Error reacting {}", why);
            }
        }
    }
}
