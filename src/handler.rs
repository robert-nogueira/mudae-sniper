use std::sync::Arc;

use serenity_self::{
    all::{ChannelId, Context, EventHandler, Message},
    async_trait,
};
use tokio::sync::{Mutex, oneshot};

use crate::{
    commands::{
        COMMAND_SCHEDULER, CollectorType, CommandContext, CommandFeedback,
        CommandType,
    },
    settings::SETTINGS,
    snipers::{SNIPERS, Sniper},
    tasks,
    utils::{InvalidStatisticsData, extract_badges, extract_statistics},
};

pub struct Handler {}

async fn setup_snipers(ctx: &Context) -> Result<(), InvalidStatisticsData> {
    let channels = SETTINGS.sniper.channels_ids.clone();
    let mut sniper: Arc<Mutex<Sniper>>;

    for channel_id in channels {
        let channel_id: ChannelId = channel_id.into();
        let (tx, rx): (
            oneshot::Sender<Option<CommandFeedback>>,
            oneshot::Receiver<Option<CommandFeedback>>,
        ) = oneshot::channel();
        let collector = COMMAND_SCHEDULER
            .default_message_collector(&ctx.shard, channel_id);
        COMMAND_SCHEDULER
            .sender()
            .send(CommandContext {
                command_type: CommandType::Tu,
                collector: CollectorType::Msg(collector),
                http: ctx.http.clone(),
                target_channel: channel_id,
                result_tx: tx,
            })
            .unwrap();
        if let Some(CommandFeedback::Msg(msg)) = rx.await.unwrap() {
            let statistics = extract_statistics(&msg.content)?;
            let collector = COMMAND_SCHEDULER
                .default_message_collector(&ctx.shard, channel_id);
            let (tx, rx): (
                oneshot::Sender<Option<CommandFeedback>>,
                oneshot::Receiver<Option<CommandFeedback>>,
            ) = oneshot::channel();
            COMMAND_SCHEDULER
                .sender()
                .send(CommandContext {
                    command_type: CommandType::Kakera,
                    collector: CollectorType::Msg(collector),
                    http: ctx.http.clone(),
                    target_channel: channel_id,
                    result_tx: tx,
                })
                .unwrap();

            if let Some(CommandFeedback::Msg(msg)) = rx.await.unwrap() {
                let badges = extract_badges(
                    &msg.embeds[0].description.clone().unwrap(),
                );
                sniper = Arc::new(Mutex::new(Sniper::new(
                    channel_id,
                    SETTINGS.sniper.guild_id.into(),
                    Arc::clone(&ctx.http),
                    ctx.shard.clone(),
                    statistics,
                    badges,
                )));
                SNIPERS.insert(channel_id, Arc::clone(&sniper));
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
                    tokio::spawn(tasks::roll_cards(
                        Arc::clone(sniper),
                        ctx.shard.clone(),
                    ));
                }
            }
        };
    }
    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == SETTINGS.client.client_id
            && msg.content.as_str() == "!start"
        {
            msg.delete(&ctx.http).await.unwrap();
            setup_snipers(&ctx).await.expect("error on setup snipers");
        };
        if !SETTINGS
            .sniper
            .channels_ids
            .contains(&msg.channel_id.into())
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
