use std::collections::HashSet;
use std::fs;

use serenity::client::Context;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::id::UserId;
use serenity::model::prelude::*;
use serenity::{async_trait, model::gateway::Ready, prelude::*};

use crate::autorole::{AutoRoleData, AutoRoleDataKey, AUTOROLE_GROUP};
use crate::commands::MY_HELP;

mod autorole;
mod commands;
mod general;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        autorole::handle_reaction(&ctx, &add_reaction)
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("token").expect("Expected bot token in file 'bot_token'");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(_) => {
            let mut owners = HashSet::new();
            owners.insert(UserId(414755070161453076)); //nils
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(false)
                .on_mention(Some(bot_id))
                .prefix("turret ")
                .delimiter(" ")
                .owners(owners)
        })
        .help(&MY_HELP)
        .group(&AUTOROLE_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<AutoRoleDataKey>(AutoRoleData::default());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
