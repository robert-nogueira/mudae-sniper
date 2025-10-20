use std::{sync::Arc, time::Duration as TimeDuration};

use serenity_self::all::{ChannelId, Http, ShardMessenger};
use tokio::{
    sync::{Mutex, oneshot},
    time::sleep,
};

use crate::{
    commands::{
        COMMAND_SCHEDULER, CommandContext, CommandFeedback, CommandType,
        FeedbackType,
    },
    snipers::{Sniper, Statistics},
    utils::get_local_time,
};

pub async fn roll_cards(
    sniper_mutex: Arc<Mutex<Sniper>>,
    shard: ShardMessenger,
) {
    let mut statistics: Statistics;
    let mut running: bool;
    let channel_id: ChannelId;
    let http: Arc<Http>;
    {
        let sniper = sniper_mutex.lock().await;
        statistics = sniper.statistics;
        running = sniper.running;
        channel_id = sniper.channel_id;
        http = sniper.http.clone();
    }

    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);
    loop {
        while !running {
            sleep(CHECK_INTERVAL).await;
            let sniper = sniper_mutex.lock().await;
            statistics = sniper.statistics;
            running = sniper.running;
        }
        let wait_duration = (statistics.next_rolls - get_local_time())
            .to_std()
            .unwrap_or(TimeDuration::ZERO);
        sleep(wait_duration).await;

        for _ in 0..statistics.rolls_remaining {
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
            if let CommandFeedback::Msg(_) = rx.await.unwrap() {
                statistics.rolls_remaining -= 1;
            }
        }
    }
}
