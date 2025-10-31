use std::{sync::Arc, time::Duration as TimeDuration};

use chrono::{DateTime, Duration};
use chrono_tz::Tz;
use log::info;
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
    utils::get_local_time,
};

pub async fn daily_claimer_task(
    sniper_mutex: Arc<Mutex<Sniper>>,
    shard: ShardMessenger,
) {
    let mut next_daily: DateTime<Tz>;
    let mut running: bool;
    {
        let sniper = sniper_mutex.lock().await;
        // info!(target: "mudae_sniper", "📝 task started: daily_claimer_task ({})", sniper.channel_id);
        info!(
            target: "mudae_sniper",
            instance:? = sniper.instance_name;
            "📝 task started: daily_claimer"
        );
        next_daily = sniper.statistics.next_daily;
        running = sniper.running;
    }

    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);

    loop {
        while !running {
            sleep(CHECK_INTERVAL).await;
            let sniper = sniper_mutex.lock().await;
            next_daily = sniper.statistics.next_daily;
            running = sniper.running;
        }

        let wait_duration = (next_daily - get_local_time())
            .to_std()
            .unwrap_or(TimeDuration::ZERO);

        sleep(wait_duration).await;
        let (channel_id, http) = {
            let sniper = sniper_mutex.lock().await;
            (sniper.instance_id, sniper.http.clone())
        };

        let (tx, rx) = oneshot::channel();
        let collector = COMMAND_SCHEDULER
            .default_reaction_collector(&shard, channel_id)
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
                target_channel: channel_id,
                result_tx: tx,
            })
            .unwrap();

        if let Some(CommandFeedback::React(_)) = rx.await.unwrap() {
            let mut sniper = sniper_mutex.lock().await;
            sniper.statistics.next_daily =
                get_local_time() + Duration::hours(20);
            next_daily = sniper.statistics.next_daily;
            running = sniper.running;
        }
    }
}
