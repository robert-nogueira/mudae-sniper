use std::{sync::Arc, time::Duration as TimeDuration};

use log::{debug, info};
use serenity_self::all::{Reaction, ReactionType, ShardMessenger};
use tokio::{
    sync::{RwLock, oneshot},
    time::sleep,
};

use crate::{
    commands::{
        COMMAND_SCHEDULER, CollectorType, CommandContext, CommandFeedback,
        CommandType,
    },
    snipers::Sniper,
    utils::{fmt_duration_from_now, get_local_time},
};

pub async fn daily_claimer_task(
    sniper_rwlock: Arc<RwLock<Sniper>>,
    shard: ShardMessenger,
) {
    let (instance, http) = {
        let sniper = sniper_rwlock.read().await;
        (sniper.instance_copy(), sniper.http.clone())
    };

    info!(
	target: "mudae_sniper",
        instance:? = instance.name;
	"📝 task started: daily_claimer");

    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);
    loop {
        while !sniper_rwlock.read().await.running {
            debug!(
                target: "mudae_sniper",
                instance:? = &instance.name;
                "🕙 task daily_claimer: instance is stopped,
                trying task again after {CHECK_INTERVAL:?}");
            sleep(CHECK_INTERVAL).await;
        }
        let statistics = sniper_rwlock.read().await.statistics_copy();

        let now = get_local_time();
        let wait_duration = (statistics.next_daily - now)
            .to_std()
            .unwrap_or(TimeDuration::ZERO);

        debug!(
            target: "mudae_sniper",
            instance:? = &instance.name;
            "⏳ waiting {} until next daily claim",
            fmt_duration_from_now(statistics.next_daily, now));

        sleep(wait_duration).await;

        let (tx, rx) = oneshot::channel();
        let collector = COMMAND_SCHEDULER
            .default_reaction_collector(&shard, instance.channel_id)
            .filter(move |r: &Reaction| match &r.emoji {
                ReactionType::Unicode(unicode) => unicode == "✅",
                _ => false,
            });

        COMMAND_SCHEDULER
            .sender()
            .send(CommandContext {
                command_type: CommandType::Daily,
                collector: CollectorType::React(collector),
                http: http.clone(),
                target_instance: instance.clone(),
                result_tx: tx,
            })
            .unwrap();

        if let Some(CommandFeedback::React(_)) = rx.await.unwrap() {
            info!(
                target: "mudae_sniper",
                instance:? = &instance.name;
                "☀️ daily claimed!");

            sniper_rwlock
                .write()
                .await
                .update_statistics()
                .await
                .expect(
                    "Failed on update statistics. Check the logs for details",
                );
        }
    }
}
