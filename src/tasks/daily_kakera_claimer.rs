use std::{sync::Arc, time::Duration as TimeDuration};

use log::{debug, info};
use serenity_self::all::ShardMessenger;
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
    utils::{REGEX_GET_NUMBERS, fmt_duration_from_now, get_local_time},
};

pub async fn daily_kakera_claimer_task(
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
	"📝 task started: daily_kakera_claimer");

    fn parse_num(text: &str) -> Option<u32> {
        text.replace(",", "").replace(".", "").parse().ok()
    }

    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);
    loop {
        while !sniper_rwlock.read().await.running {
            debug!(
                target: "mudae_sniper",
                instance:? = &instance.name;
                "🕙 task daily_kakera_claimer: instance is stopped, trying task again after {CHECK_INTERVAL:?}"
            );
            sleep(CHECK_INTERVAL).await;
        }
        let statistics = sniper_rwlock.read().await.statistics_copy();

        let now = get_local_time();
        let wait_duration = (statistics.next_dk - now)
            .to_std()
            .unwrap_or(TimeDuration::ZERO);

        debug!(
            target: "mudae_sniper",
            instance:? = &instance.name;
            "⏳ waiting {} until next daily kakera claim",
            fmt_duration_from_now(statistics.next_dk, now));

        sleep(wait_duration).await;

        let (tx, rx) = oneshot::channel();
        let collector = COMMAND_SCHEDULER
            .default_message_collector(&shard, instance.channel_id)
            .filter(move |m| REGEX_GET_NUMBERS.is_match(&m.content));

        COMMAND_SCHEDULER
            .sender()
            .send(CommandContext {
                command_type: CommandType::DailyKakera,
                collector: CollectorType::Msg(collector),
                http: http.clone(),
                target_instance: instance.clone(),
                result_tx: tx,
            })
            .unwrap();

        if let Some(CommandFeedback::Msg(msg)) = rx.await.unwrap() {
            let claimed_kakera = REGEX_GET_NUMBERS
                .find_iter(&msg.content)
                .next()
                .and_then(|m| parse_num(m.as_str()));

            info!(
                target: "mudae_sniper",
                instance:? = &instance.name;
                "✨ claimed kakera: {}", claimed_kakera.unwrap());

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
