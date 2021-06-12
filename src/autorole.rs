use serenity::framework::standard::Args;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[group]
struct AutoRole;

#[command]
#[desciption = "Add auto assign roles"]
async fn auto_role(ctx: &Context, msg: &Message, args: &Args) {}
