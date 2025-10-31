use std::sync::Arc;

use log::info;
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
    let channels = &SETTINGS.sniper.channels;
    let mut sniper: Arc<Mutex<Sniper>>;
    let channels_amount = channels.len();
    for (i, channel_cfg) in SETTINGS.sniper.channels.iter().enumerate() {
        let index = i + 1;
        let channel_id: ChannelId = channel_cfg.id.into();
        let (tx, rx): (
            oneshot::Sender<Option<CommandFeedback>>,
            oneshot::Receiver<Option<CommandFeedback>>,
        ) = oneshot::channel();
        let collector = COMMAND_SCHEDULER
            .default_message_collector(&ctx.shard, channel_cfg.id.into());
        COMMAND_SCHEDULER
            .sender()
            .send(CommandContext {
                command_type: CommandType::Tu,
                collector: CollectorType::Msg(collector),
                http: ctx.http.clone(),
                target_channel: channel_cfg.id.into(),
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
                    channel_cfg.name.clone(),
                )));
                SNIPERS.insert(channel_id, Arc::clone(&sniper));
                info!(
                    target: "mudae_sniper",
                channel_name:? = channel_cfg.name,
                channel_id = u64::from(channel_id);
                "⚙️ sniper for channel configured {index}/{}",
                channels_amount
                );
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
    info!(target: "mudae_sniper",  "✅ snipers setup complete.");
    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == SETTINGS.client.client_id
            && msg.content.as_str() == "!start"
        {
            info!(
            target: "mudae_sniper",
            "Start command detected, setting up snipers..."
            );
            msg.delete(&ctx.http).await.unwrap();
            setup_snipers(&ctx).await.expect("error on setup snipers");
        };
        let chan_id: u64 = msg.channel_id.into();

        if !SETTINGS.sniper.channels.iter().any(|c| c.id == chan_id)
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
