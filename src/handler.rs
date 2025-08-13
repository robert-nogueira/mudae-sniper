use std::{sync::Arc, time::Duration};

use serenity_self::{
    all::{Context, EventHandler, Message, MessageCollector, async_trait},
    futures::StreamExt,
};
use tokio::sync::Mutex;

use crate::{
    settings::SETTINGS,
    snipers::{SNIPERS, Sniper},
};

pub struct Handler {}

async fn setup_snipers(ctx: &Context) {
    let channels = SETTINGS.channels_ids.clone();
    let mut sniper: Arc<Mutex<Sniper>>;
    for channel_id in channels {
        sniper = Arc::new(Mutex::new(Sniper::new(
            channel_id.into(),
            SETTINGS.guild_id.into(),
            Arc::clone(&ctx.http),
        )));
        SNIPERS.insert(channel_id, Arc::clone(&sniper));
        let mut sniper = sniper.lock().await;
        let command = sniper.channel_id.say(&sniper.http, "$tu").await.unwrap();
        let mut collector = MessageCollector::new(ctx)
            .channel_id(sniper.channel_id)
            .author_id(432610292342587392.into())
            .timeout(Duration::from_secs(30))
            .filter(move |m: &Message| m.content.contains(&command.author.name))
            .stream();
        if let Some(msg) = collector.next().await {
            match sniper.update_statistics(&msg.content) {
                Ok(_) => sniper.running = true,
                Err(_) => {
                    msg.react(&sniper.http, '❌').await.unwrap();
                }
            };
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == SETTINGS.client_id && msg.content.as_str() == "!start" {
            msg.delete(&ctx.http).await.unwrap();
            setup_snipers(&ctx).await;
        };
        let channel_id: u64 = msg.channel_id.into();
        if !SETTINGS.channels_ids.contains(&channel_id) || msg.author.id != 432610292342587392 {
            return;
        }

        if let Some(sniper) = SNIPERS.get(&channel_id) {
            let sniper = sniper.lock().await;
            sniper.snipe_kakera(&ctx, &msg).await;
        }
    }
}
