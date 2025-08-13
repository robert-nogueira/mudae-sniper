use std::sync::Arc;

use serenity_self::all::{Context, EventHandler, Http, Message, async_trait};
use tokio::sync::Mutex;

use crate::{
    settings::SETTINGS,
    snipers::{SNIPERS, Sniper},
};

pub struct Handler {}

fn setup_snipers(http: Arc<Http>) {
    let channels = SETTINGS.channels_ids.clone();
    let mut sniper: Arc<Mutex<Sniper>>;
    for channel_id in channels {
        sniper = Arc::new(Mutex::new(Sniper::new(
            channel_id,
            SETTINGS.guild_id,
            Arc::clone(&http),
        )));
        SNIPERS.insert(channel_id, Arc::clone(&sniper));
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == SETTINGS.client_id && msg.content.as_str() == "!start" {
            setup_snipers(Arc::clone(&ctx.http))
        }
        if let Some(sniper) = SNIPERS.get(&msg.channel_id.into()) {
            let sniper = sniper.lock().await;

            if !sniper.running {
                return;
            }

            let my_id = SETTINGS.client_id;
            if msg.author.id == my_id {
                return;
            }
            if let Some(kakera) = sniper.snipe_kakera(&ctx, &msg).await {
                println!("Catch: {kakera}")
            }
        }
    }
}
