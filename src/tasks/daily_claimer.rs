use std::{sync::Arc, time::Duration as TimeDuration};

use chrono::{DateTime, Duration};
use chrono_tz::Tz;
use log::{debug, info};
use serenity_self::all::{Reaction, ReactionType, ShardMessenger};
use tokio::{
    sync::{Mutex, oneshot},
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
    sniper_mutex: Arc<Mutex<Sniper>>,
    shard: ShardMessenger,
) {
    let mut next_daily: DateTime<Tz>;
    let mut running: bool;
    let instance;
    {
        let sniper = sniper_mutex.lock().await;
        instance = sniper.instance_copy();
        info!(
            target: "mudae_sniper",
            instance:? = sniper.instance_ref().name;
            "📝 task started: daily_claimer"
        );
        next_daily = sniper.statistics_ref().next_daily;
        running = sniper.running;
    }
    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);
    loop {
        while !running {
            debug!(
                target: "mudae_sniper",
                instance:? = &instance.name;
                "🕙 task daily_claimer: instance is stopped, trying task again after {CHECK_INTERVAL:?}"
            );
            sleep(CHECK_INTERVAL).await;
            let sniper = sniper_mutex.lock().await;
            next_daily = sniper.statistics_ref().next_daily;
            running = sniper.running;
        }
        let now = get_local_time();
        let wait_duration =
            (next_daily - now).to_std().unwrap_or(TimeDuration::ZERO);
        debug!(
            target: "mudae_sniper",
            instance:? = &instance.name;
            "⏳ waiting {} until next daily claim",
            fmt_duration_from_now(next_daily, now)
        );
        sleep(wait_duration).await;
        let http = {
            let sniper = sniper_mutex.lock().await;
            sniper.http.clone()
        };
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
                "☀️ daily claimed!"
            );
            let mut sniper = sniper_mutex.lock().await;
            sniper.update_statistics().await.expect(
                "Failed on update statistics. Check the logs for details",
            );
            next_daily = sniper.statistics_ref().next_daily;
            running = sniper.running;
        }
    }
}
