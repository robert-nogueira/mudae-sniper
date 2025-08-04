mod handler;
mod models;
mod settings;
mod sniper;

use std::{collections::HashMap, sync::Arc};

use handler::Handler;
use serenity_self::{Client, all::GatewayIntents};
use settings::Settings;
use sniper::Sniper;

#[tokio::main]
async fn main() {
    let settings = Arc::new(Settings::load());
    let mut snipers = HashMap::new();
    let channels = settings.channels_ids.clone();
    for channel_id in channels {
        snipers.insert(
            channel_id,
            Sniper::new(channel_id, settings.guild_id, Arc::clone(&settings)),
        );
    }
    let handler = Handler {
        snipers,
        settings: Arc::clone(&settings),
    };

    let mut client = Client::builder(&settings.token, GatewayIntents::all())
        .event_handler(handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Err creating client: {why}");
    };
}
