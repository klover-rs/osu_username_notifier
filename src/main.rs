use std::process;
use tokio::try_join;

mod discord_bot;

#[tokio::main]
async fn main() -> Result<(), String> {

    let discord_bot_task = tokio::spawn(discord_bot::bot::main());

    if let Err(e) = try_join!(discord_bot_task) {

        process::exit(1);
    }

    Ok(())
}

