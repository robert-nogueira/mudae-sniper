mod commands;
mod entities;
mod handler;
mod settings;
mod snipers;
mod tasks;
mod utils;

use handler::Handler;
use serenity_self::{Client, all::GatewayIntents};
use settings::SETTINGS;

#[tokio::main]
async fn main() {
    let handler = Handler {};

    let mut client =
        Client::builder(SETTINGS.client.token.clone(), GatewayIntents::all())
            .event_handler(handler)
            .await
            .expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Err creating client: {why}");
    };
}
