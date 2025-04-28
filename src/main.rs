mod discord;
mod minecraft;

extern crate dotenv;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let discord_token = std::env::var("DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found");

    discord::run_bot(discord_token).await;
}
