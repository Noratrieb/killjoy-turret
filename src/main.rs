mod autorole;
mod commands;
mod general;

use anyhow::{bail, Context as _};
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
use tracing::{info, Level};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_ansi(true)
        .with_thread_names(false)
        .with_max_level(Level::INFO)
        .init();

    let file_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "./config.json".to_string());
    let file = fs::read_to_string(&file_path)
        .context("Error reading config file")
        .context(file_path)?;

    let config = serde_json::from_str::<ConfigFile>(&file).context("parse config")?;

    let token = &config.token;

    let http = Http::new_with_token(token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(_) => {
            let mut owners = HashSet::new();
            owners.insert(UserId(414755070161453076)); //nils
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => bail!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => bail!("Could not access application info: {:?}", why),
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
        .context("creating client")?;

    {
        let mut data = client.data.write().await;
        data.insert::<AutoRoleDataKey>(AutoRoleData::default());
        data.insert::<ConfigFile>(config);
    }

    client.start().await.context("running bot")?;

    Ok(())
}
