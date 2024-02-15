use poise::FrameworkContext;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::gateway::ActivityData;
use serenity::model::user::OnlineStatus;

use crate::discord_bot::bot::{Data, Error};

pub async fn handle_ready(data_about_bot: &Ready, _framework: FrameworkContext<'_, Data, Error>, ctx: &Context) -> Result<(), Error> {
    println!("Logged in as {}", data_about_bot.user.name);
    
    let activity = ActivityData::watching("you");

    let status = OnlineStatus::Idle;

    ctx.set_presence(Some(activity), status);

    Ok(())
}

