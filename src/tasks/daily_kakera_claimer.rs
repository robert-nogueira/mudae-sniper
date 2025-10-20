use std::{sync::Arc, time::Duration as TimeDuration};

use chrono::{DateTime, Duration};
use chrono_tz::Tz;
use serenity_self::all::ShardMessenger;
use tokio::{
    sync::{Mutex, oneshot},
    time::sleep,
};

use crate::{
    commands::{
        COMMAND_SCHEDULER, CommandContext, CommandFeedback, CommandType,
        FeedbackType,
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
        next_dk = sniper.statistics.next_dk;
        running = sniper.running;
    }

    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);

    loop {
        while !running {
            sleep(CHECK_INTERVAL).await;
            let sniper = sniper_mutex.lock().await;
            next_dk = sniper.statistics.next_dk;
            running = sniper.running;
        }

        let wait_duration = (next_dk - get_local_time())
            .to_std()
            .unwrap_or(TimeDuration::ZERO);

        sleep(wait_duration).await;

        let (channel_id, http) = {
            let sniper = sniper_mutex.lock().await;
            (sniper.channel_id, sniper.http.clone())
        };

        let (tx, rx) = oneshot::channel();
        {
            COMMAND_SCHEDULER
                .schedule_command(CommandContext {
                    command_type: CommandType::DailyKakera,
                    expected_feedback: FeedbackType::Reaction,
                    http: http.clone(),
                    shard: shard.clone(),
                    target_channel: channel_id,
                    result_tx: tx,
                })
                .await;
        }
        if let CommandFeedback::Msg(msg) = rx.await.unwrap() {
            let stock_value = REGEX_GET_NUMBERS
                .find_iter(&msg.content)
                .next()
                .and_then(|m| parse_num(m.as_str()));

            let mut sniper = sniper_mutex.lock().await;
            sniper.statistics.kakera_stock =
                stock_value.unwrap_or(sniper.statistics.kakera_stock);
            sniper.statistics.next_dk = get_local_time() + Duration::hours(20);
            next_dk = sniper.statistics.next_dk;
            running = sniper.running;
        }
    }
}
