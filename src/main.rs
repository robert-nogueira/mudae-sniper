mod handler;
mod models;
mod settings;
mod snipers;

use std::collections::HashMap;

use handler::Handler;
use serenity_self::{Client, all::GatewayIntents};
use settings::SETTINGS;
use snipers::Sniper;

#[tokio::main]
async fn main() {
    let mut snipers = HashMap::new();
    let channels = SETTINGS.channels_ids.clone();
    for channel_id in channels {
        snipers.insert(channel_id, Sniper::new(channel_id, SETTINGS.guild_id));
    }
    let handler = Handler { snipers };

    let mut client = Client::builder(SETTINGS.token.clone(), GatewayIntents::all())
        .event_handler(handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Err creating client: {why}");
    };
}
