mod autorole;
mod commands;
mod general;

use std::collections::HashSet;
use std::fs;

use crate::autorole::{AutoRoleData, AutoRoleDataKey};
use crate::commands::MY_HELP;
use crate::general::{normal_message, ConfigFile};
use serenity::client::Context;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::id::UserId;
use serenity::{async_trait, model::gateway::Ready, prelude::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let file_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "./config.json".to_string());
    println!("reading config from {file_path}");
    let file = fs::read_to_string(file_path).unwrap();
    let config = serde_json::from_str::<ConfigFile>(&file).unwrap();

    let token = &config.token;

    let http = Http::new_with_token(token);

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
        .normal_message(normal_message);
    // .group(&AUTOROLE_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<AutoRoleDataKey>(AutoRoleData::default());
        data.insert::<ConfigFile>(config);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
