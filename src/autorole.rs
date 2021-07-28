use serde::{Deserialize, Serialize};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    async_trait,
    collector::MessageCollectorBuilder,
    framework::standard::{help_commands, CommandGroup, HelpOptions, StandardFramework},
    futures::stream::StreamExt,
    http::Http,
    model::prelude::*,
    prelude::*,
};
use std::collections::HashMap;
use std::time::Duration;

pub struct AutoRoleDataKey;

impl TypeMapKey for AutoRoleDataKey {
    type Value = AutoRoleData;
}

#[group]
#[commands(autorole)]
#[description = "Auto role related commands"]
struct AutoRole;

#[command]
#[description = "Add auto assign roles"]
#[usage = "sdakhnfj"]
async fn autorole(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let http = &ctx.http;
    let mut data = ctx.data.write().await;
    let mut auto_roles = data.get_mut::<AutoRoleDataKey>();

    msg.channel_id
        .send_message(http, |m| {
            m.embed(|e| {
                e.title("Started auto role assignment.");
                e.field(
                    "Enter an emote and the assigned role seperated by a space.",
                    "Write one message per role reaction.\n\n\
                    A non-matching message finishes the process. Cancel with 'cancel', or after 30 seconds timeout\n\n\
                    The emotes have to be from a server I'm in!",
                    false,
                )
            })
        })
        .await?;

    while let Some(answer) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(30)).await {
        if let Some(emote) = answer.content.split(' ').next() {
            lazy_static! {}

            println!("should add emote: {}", emote);
            answer.react(http, '☑').await?;
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct AutoRoleData {
    messages: Vec<HashMap<DEmote, DRole>>,
}

impl Default for AutoRoleData {
    fn default() -> Self {
        Self {
            messages: Vec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
struct DEmote {
    name: String,
    id: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
struct DRole {
    name: String,
    id: String,
}
