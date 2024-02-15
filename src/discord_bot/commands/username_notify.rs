use poise::CreateReply;
use serenity::builder::CreateEmbed;
use reqwest::Client;
use serde_json::Value;
use std::time::{Duration, Instant};

use crate::discord_bot::bot::Context;
use crate::discord_bot::bot::Error;

use serenity::builder::{CreateMessage, EditMessage};

const ALLOWED_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

#[poise::command(prefix_command, slash_command)]
pub async fn username_availablity(
    ctx: Context<'_>,
    #[description = "the username you want to check for."]
    username: String,
    #[description = "enter here your osu cookie (this is used to make a request to the osu server)"]
    cookie: String,
) -> Result<(), Error> {
    let embed = CreateEmbed::default()
        .title("please make sure to have your dms turned on for this server!");


    if !is_valid_username(&username) {
        ctx.send(CreateReply::default().embed(
            CreateEmbed::default()
                .title("Username Requirements")
                .field("Allowed Characters", "Alphabets: A-Z, a-z\nNumbers: 0-9\nSpecial Characters: Hyphen (-), Underscore (_)", false)
                .field(
                    "Rules",
                    "Usernames must contain at least one character.\nUsernames must not exceed 16 characters in length.\n
                    Usernames can contain at most one space, but leading and trailing spaces are not allowed.\n
                    Usernames can include letters, numbers, hyphens, and underscores, but no other special characters are permitted.",false
                )
        ).ephemeral(true)).await?;
        return Ok(())
    }

    let (xsrf_token, osu_session) = extract_tokens(&cookie);

    let (xsrf, session) = match (xsrf_token, osu_session) {
        (Some(xsrf), Some(session)) => {
            // Handle the first case here if needed
            (xsrf, session)
        }
        (Some(xsrf), None) => {
            println!("XSRF-TOKEN: {}", xsrf);
            println!("osu_session not found.");
            ctx.send(CreateReply::default().content("wrong format")).await?;
            return Ok(());
        }
        (None, Some(session)) => {
            println!("XSRF-TOKEN not found.");
            println!("osu_session: {}", session);
            ctx.send(CreateReply::default().content("wrong format")).await?;
            return Ok(())
        }
        (None, None) => {
            println!("Neither XSRF-TOKEN nor osu_session found.");
            ctx.send(CreateReply::default().content("wrong format")).await?;
            return Ok(())
        }
    };

    ctx.send(CreateReply::default().embed(embed)).await?;

    let dm_channel = ctx.author().create_dm_channel(&ctx).await?;

    let sent_message = dm_channel.send_message(
        &ctx,
        CreateMessage::default().content("starting.. please wait")
    ).await?;

    let start_time = Instant::now();    
    loop {

        let elapsed = start_time.elapsed();

        let seconds = elapsed.as_secs() % 60;
        let minutes = (elapsed.as_secs() / 60) % 60;
        let hours = (elapsed.as_secs() / 3600) % 24;
        let days = elapsed.as_secs() / (3600 * 24);

        println!(
            "Elapsed time: {} days, {} hours, {} minutes, {} seconds",
            days, hours, minutes, seconds
        );

        match fetch_username_check(&username, &xsrf, &session).await {
            Ok(true) => {
                dm_channel.send_message(&ctx, CreateMessage::default().embed(CreateEmbed::default()
                    .title("THE USERNAME IS NOW AVAILABLE :3")
                    .description("congratualtion, this worked, your username is now available, claim it as fast as possible or else it might be taken by someone else !!\n\nclick on the blue title to get redirected to the osu player name change page\n\nthank you for using this project, i appreciate it!")
                    .url("https://osu.ppy.sh/store/products/32")
                )).await?;
                break
            }
            Ok(false) => {
                dm_channel.edit_message(&ctx, sent_message.id, EditMessage::default().content(
                    format!("elapsed: `{} : {} : {} : {}`\nonce your username is available, you will be notified via direct messages.", days, hours, minutes, seconds)
                )).await?;
                false
            }
            Err(e) => {
                dm_channel.send_message(&ctx, CreateMessage::default().content(format!("Oops! seems like there occurred an error!: {}", e))).await?;
                break
            }
        };

        std::thread::sleep(Duration::from_secs(4));

        
    }    
    
    Ok(())
}

fn is_valid_username(username: &str) -> bool {
    let mut space_count = 0;

    for c in username.chars() {
        if c == ' ' {
            space_count += 1;

            if space_count > 1 {
                return false;
            }
        } else if !ALLOWED_CHARS.contains(c) {
            return false
        }
    }

    if username.len() >= 16 {
        return false;
    }

    !username.is_empty() && !username.ends_with(' ') &&!username.starts_with(' ')
}

async fn fetch_username_check(username: &str, xsrf_token: &str, cookie: &str) -> Result<bool, Box<dyn std::error::Error + Send>> {
    let client = Client::new();


    let response = client
        .post("https://osu.ppy.sh/users/check-username-availability")
        .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
        .header("cookie", format!("XSRF-TOKEN={}; osu_session={}", xsrf_token, cookie))
        .header("x-csrf-token", format!("{}", xsrf_token))
        .header("referer", "https://osu.ppy.sh/store/products/32")
        .header("referrerPolicy", "strict-origin-when-cross-origin")
        .body(format!("username={}", username))
        .send()
        .await.unwrap();

    if !response.status().is_success() {
        eprintln!("Failed to fetch");
        return Ok(false);
    }

    let body: Value = response.json().await.unwrap();
    
    let available = body["available"].as_bool().unwrap_or(false);

    println!("{:?}", body);

    println!("{available}");

    Ok(available)
}

fn extract_tokens(input: &str) -> (Option<&str>, Option<&str>) {
    let mut xsrf_token = None;
    let mut osu_session = None;

    let parts: Vec<&str> = input.split(';').collect();

    for part in parts {
        let tokens: Vec<&str> = part.trim().split('=').collect();

        if tokens.len() == 2 {
            match tokens[0] {
                "XSRF-TOKEN" => xsrf_token = Some(tokens[1]),
                "osu_session" => osu_session = Some(tokens[1]),
                _ => {}
            }
        }
    }

    
    (xsrf_token, osu_session)
}