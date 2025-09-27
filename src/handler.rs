use std::{sync::Arc, time::Duration};

use serenity_self::{
    all::{
        ChannelId, Context, EventHandler, Message, MessageCollector,
        async_trait,
    },
    futures::StreamExt,
};
use tokio::sync::Mutex;

use crate::{
    settings::SETTINGS,
    snipers::{SNIPERS, Sniper},
    tasks,
    utils::extract_statistics,
};

pub struct Handler {}

async fn setup_snipers(ctx: &Context) {
    let channels = SETTINGS.channels_ids.clone();
    let mut sniper: Arc<Mutex<Sniper>>;
    for channel_id in channels {
        let channel_id: ChannelId = channel_id.into();
        let command = channel_id.say(&ctx, "$tu").await.unwrap();
        let mut collector = MessageCollector::new(ctx)
            .channel_id(channel_id)
            .author_id(432610292342587392.into())
            .timeout(Duration::from_secs(30))
            .filter(move |m: &Message| {
                m.content.contains(&command.author.name)
            })
            .stream();
        if let Some(msg) = collector.next().await {
            let statistics = extract_statistics(&msg.content);
            match statistics {
                Ok(statistics) => {
                    sniper = Arc::new(Mutex::new(Sniper::new(
                        channel_id,
                        SETTINGS.guild_id.into(),
                        Arc::clone(&ctx.http),
                        statistics,
                    )));
                    SNIPERS.insert(channel_id, Arc::clone(&sniper));
                }
                Err(_) => {
                    msg.react(&ctx, '❌').await.unwrap();
                }
            };
            for entry in SNIPERS.iter() {
                let sniper = entry.value();
                tokio::spawn(tasks::daily_claimer_task(
                    Arc::clone(sniper),
                    ctx.shard.clone(),
                ));
                tokio::spawn(tasks::daily_kakera_claimer_task(
                    Arc::clone(sniper),
                    ctx.shard.clone(),
                ));
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == SETTINGS.client_id
            && msg.content.as_str() == "!start"
        {
            msg.delete(&ctx.http).await.unwrap();
            setup_snipers(&ctx).await;
        };
        if !SETTINGS.channels_ids.contains(&msg.channel_id.into())
            || msg.author.id != 432610292342587392
        {
            return;
        }
        if let Some(sniper) = SNIPERS.get(&msg.channel_id) {
            let mut sniper = sniper.lock().await;
            sniper.snipe_kakeras(&msg).await;
        }
    }
}
