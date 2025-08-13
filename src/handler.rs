use std::{sync::Arc, time::Duration};

use serenity_self::{
    all::{
        CacheHttp, ChannelId, Context, EventHandler, Http, Message, MessageCollector, async_trait,
    },
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
        let mut collector = MessageCollector::new(&ctx.shard)
            .channel_id(sniper.channel_id)
            .author_id(432610292342587392.into())
            .timeout(Duration::from_secs(30))
            .stream();
        if let Some(msg) = collector.next().await {
            if sniper.update_statistics(&msg.content).is_err() {
                command.react(&sniper.http, '❌').await.unwrap();
            };
        } else {
            command.react(&sniper.http, '❌').await.unwrap();
        }
        command.delete(&sniper.http).await.unwrap();
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == SETTINGS.client_id && msg.content.as_str() == "!start" {
            setup_snipers(&ctx).await;
        };
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
