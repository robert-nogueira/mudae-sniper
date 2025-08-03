mod handler;
mod models;
mod settings;
mod sniper;

use std::collections::HashMap;

use handler::Handler;
use serenity_self::{Client, all::GatewayIntents};
use settings::Settings;
use sniper::Sniper;

#[tokio::main]
async fn main() {
    let settings = Settings::load();
    let mut snipers = HashMap::new();
    for channel_id in settings.channels_ids {
        snipers.insert(channel_id, Sniper::new(channel_id, settings.guild_id).await);
    }
    let handler = Handler { snipers };

    let mut client = Client::builder(&settings.token, GatewayIntents::all())
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Err creating client: {why}");
    };
}
