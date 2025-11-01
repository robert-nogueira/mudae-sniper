use std::{sync::Arc, time::Duration as TimeDuration};

use chrono::{DateTime, Duration};
use chrono_tz::Tz;
use log::info;
use serenity_self::all::ShardMessenger;
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
    utils::{REGEX_GET_NUMBERS, get_local_time},
};

pub async fn daily_kakera_claimer_task(
    sniper_mutex: Arc<Mutex<Sniper>>,
    shard: ShardMessenger,
) {
    fn parse_num(text: &str) -> Option<u32> {
        text.replace(",", "").replace(".", "").parse().ok()
    }

    let mut next_dk: DateTime<Tz>;
    let mut running: bool;
    {
        let sniper = sniper_mutex.lock().await;
        info!(
                target: "mudae_sniper",
                instance:? = sniper.instance_ref().name;
                "📝 task started: daily_kakera_claimer"
        );
        next_dk = sniper.statistics_ref().next_dk;
        running = sniper.running;
    }

    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);

    loop {
        while !running {
            sleep(CHECK_INTERVAL).await;
            let sniper = sniper_mutex.lock().await;
            next_dk = sniper.statistics_ref().next_dk;
            running = sniper.running;
        }

        let wait_duration = (next_dk - get_local_time())
            .to_std()
            .unwrap_or(TimeDuration::ZERO);

        sleep(wait_duration).await;

        let (instance, http) = {
            let sniper = sniper_mutex.lock().await;
            (sniper.instance_copy(), sniper.http.clone())
        };

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
                target_instance: instance,
                result_tx: tx,
            })
            .unwrap();

        if let Some(CommandFeedback::Msg(msg)) = rx.await.unwrap() {
            let claimed_kakera = REGEX_GET_NUMBERS
                .find_iter(&msg.content)
                .next()
                .and_then(|m| parse_num(m.as_str()));
            let mut sniper = sniper_mutex.lock().await;
            sniper.update_statistics().await;
            next_dk = sniper.statistics_ref().next_dk;
            running = sniper.running;
            info!(
            target: "mudae_sniper",
            instance:? = sniper.instance_ref().name;
            "✨ claimed kakera: {}", claimed_kakera.unwrap()
            );
        }
    }
}
