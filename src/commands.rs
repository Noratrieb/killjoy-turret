use std::collections::HashSet;

use serenity::client::Context;
use serenity::framework::standard::{
    help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;

#[help]
#[individual_command_tip = "Killjoy Turret Command Info"]
#[command_not_found_text = "Could not find command: `{}`."]
#[max_levenshtein_distance(3)]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = ""]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
