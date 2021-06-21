use crate::Handler;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::ReactionType::Custom;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
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
                match role.to_role_cached(&ctx).await {
                    Some(role) => match EmojiIdentifier::from_str(emote) {
                        Ok(emojiIdent) => roles.push((emojiIdent, role)),
                        Err(_) => {
                            answer.channel_id.say(http, "Could not find emoji").await?;
                        }
                    },
                    None => {
                        answer
                            .channel_id
                            .say(http, "Could not find role, try again or use another role")
                            .await?;
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if roles.len() == 0 {
        msg.channel_id.say(http, "No roles were entered.").await?;
        return Ok(());
    }

    msg.channel_id.say(http, "done").await?;

    let reaction_message = target_channel
        .send_message(http, |m| {
            m.embed(|e| {
                e.title(message).fields(
                    roles
                        .iter()
                        .map(|(emote, role)| (&role.name, format!("<:{}:{}>", emote.name, emote.id), true)),
                )
            })
        })
        .await?;

    for data in &roles {
        reaction_message.react(http, data.0.clone()).await?;
    }

    let mut data = ctx.data.write().await;
    let mut auto_roles = data.get_mut::<AutoRoleDataKey>().unwrap();

    roles
        .iter()
        .map(|(emoji, role)| ((emoji.id, reaction_message.id), role.id))
        .for_each(|(key, value)| {
            auto_roles.messages.insert(key, value);
        });

    Ok(())
}

// listener

pub async fn handle_reaction(ctx: &Context, reaction: &Reaction) {
    let data = ctx.data.read().await;
    let mut auto_roles = data.get::<AutoRoleDataKey>().unwrap();
    if let Custom { animated, id, name } = &reaction.emoji {
        if let Some(role) = auto_roles.messages.get(&(*id, reaction.message_id)) {
            match &reaction.member {
                None => {
                    println!("Missing member on reaction");
                    // remove reaction
                }
                Some(member) => {
                    // add role
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AutoRoleData {
    messages: HashMap<(EmojiId, MessageId), RoleId>,
}

impl Default for AutoRoleData {
    fn default() -> Self {
        Self {
            messages: HashMap::default(),
        }
    }
}
