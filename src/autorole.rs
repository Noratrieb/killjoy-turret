use serde::{Deserialize, Serialize};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
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

const DEFAULT_MESSAGE: &'static str = "Select your role";

#[command]
#[aliases("a")]
#[description = "Add auto assign roles"]
#[usage = "sdakhnfj"]
async fn autorole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let http = &ctx.http;

    let target_channel = args.single::<ChannelId>();
    args.quoted();
    let message = args.single::<String>().unwrap_or(DEFAULT_MESSAGE.to_string());

    if let Err(_) = target_channel {
        msg.channel_id
            .say(http, "You need to mention the channel the message should be sent in")
            .await?;
        return Ok(());
    }
    let target_channel = target_channel.unwrap();

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

    let mut roles = vec![];

    while let Some(answer) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(30)).await {
        if answer.content.contains("cancel") {
            return Ok(());
        }
        if let Some(role) = answer.mention_roles.get(0) {
            if let Some(emote) = answer.content.split(' ').next() {
                println!("should add emote: {} for role {}", emote, role.0);
                answer.react(http, '☑').await?;

                roles.push((emote.to_string(), role.0));
            } else {
                break;
            }
        } else {
            break;
        }
    }

    msg.channel_id.say(http, "done").await?;

    target_channel
        .send_message(http, |m| {
            m.embed(|e| {
                e.title(message);
                e.field("neutral", "same", false)
            })
        })
        .await?;

    let mut data = ctx.data.write().await;
    let mut auto_roles = data.get_mut::<AutoRoleDataKey>().unwrap();

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
