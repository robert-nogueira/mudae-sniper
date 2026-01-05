use std::sync::Arc;

use log::{error, info};
use serenity_self::{
    all::{ChannelId, Context, EventHandler, Message},
    async_trait,
};
use tokio::sync::{RwLock, oneshot};

use crate::{
    commands::{
        COMMAND_SCHEDULER, CollectorType, CommandContext, CommandFeedback,
        CommandType,
    },
    entities::instance::Instance,
    settings::SETTINGS,
    snipers::{SNIPERS, Sniper, errors::CaptureError},
    tasks,
    utils::{
        InvalidStatisticsData, extract_badges, extract_kakera_value,
        extract_statistics, get_local_time,
    },
};

pub struct Handler {}

async fn setup_snipers(ctx: &Context) -> Result<(), InvalidStatisticsData> {
    let channels = &SETTINGS.sniper.instances;
    let mut sniper: Arc<RwLock<Sniper>>;
    let channels_amount = channels.len();
    for (i, instance_cfg) in SETTINGS.sniper.instances.iter().enumerate() {
        let instance = Instance {
            channel_id: instance_cfg.id.into(),
            name: instance_cfg.name.clone(),
            roll_after_claim: instance_cfg.roll_after_claim,
        };
        let index = i + 1;
        let channel_id: ChannelId = instance_cfg.id.into();
        let (tx, rx): (
            oneshot::Sender<Option<CommandFeedback>>,
            oneshot::Receiver<Option<CommandFeedback>>,
        ) = oneshot::channel();
        let collector = COMMAND_SCHEDULER
            .default_message_collector(&ctx.shard, instance_cfg.id.into());
        COMMAND_SCHEDULER
            .sender()
            .send(CommandContext {
                command_type: CommandType::Tu,
                collector: CollectorType::Msg(collector),
                http: ctx.http.clone(),
                target_instance: instance.clone(),
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
                    target_instance: instance.clone(),
                    result_tx: tx,
                })
                .unwrap();

            if let Some(CommandFeedback::Msg(msg)) = rx.await.unwrap() {
                let badges = extract_badges(
                    &msg.embeds[0].description.clone().unwrap(),
                );
                sniper = Arc::new(RwLock::new(Sniper::new(
                    SETTINGS.sniper.guild_id.into(),
                    Arc::clone(&ctx.http),
                    ctx.shard.clone(),
                    statistics,
                    badges,
                    instance,
                )));
                SNIPERS.insert(channel_id, Arc::clone(&sniper));
                info!(
                    target: "mudae_sniper",
                    instance_name:? = instance_cfg.name,
                    instance_id = u64::from(channel_id);
                    "⚙️ sniper for channel configured {index}/{}",
                    channels_amount
                );
            }
        };
    }
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
        tokio::spawn(tasks::roll_cards(Arc::clone(sniper), ctx.shard.clone()));
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
                "Start command detected, setting up snipers...");
            msg.delete(&ctx.http).await.unwrap();
            setup_snipers(&ctx).await.expect("error on setup snipers");
        };

        if msg.author.id != 432610292342587392
            || msg.embeds.is_empty()
            || msg.embeds[0].description.is_none()
            || msg.components.is_empty()
        {
            return;
        }

        let Some(sniper_rwlock) = SNIPERS.get(&msg.channel_id) else {
            return;
        };
        let (stats, instance) = {
            let sniper = sniper_rwlock.read().await;
            (sniper.statistics_copy(), sniper.instance_copy())
        };

        let now = get_local_time();
        if stats.next_kakera_react <= now {
            sniper_rwlock.write().await.snipe_kakeras(&msg).await;
        }
        let Some(kakera_value) = extract_kakera_value(&msg.embeds[0]) else {
            return;
        };

        if kakera_value >= SETTINGS.sniper.capture_threshold
            && stats.can_claim
            && let Err(error) =
                sniper_rwlock.write().await.capture_card(&msg).await
        {
            match error {
                CaptureError::InvalidButton(button) => {
                    error!(target: "mudae_sniper::capture",
			   "🚨 [{}] invalid claim button: {:?}",
			   instance.name, button);
                }
                CaptureError::NotAButton(component) => {
                    error!(target: "mudae_sniper::capture",
			   "🚨 [{}] component is not a button: {:?}",
			   instance.name, component);
                }
                CaptureError::MissingComponent => {
                    error!(target: "mudae_sniper::capture",
			   "🚨 [{}] missing component while trying to capture card",
			   instance.name);
                }
            }
        }
    }
}
