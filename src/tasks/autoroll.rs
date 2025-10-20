use std::{sync::Arc, time::Duration as TimeDuration};

use serenity_self::all::{ChannelId, Http, ShardMessenger};
use tokio::{
    sync::{Mutex, oneshot},
    time::sleep,
};

use crate::{
    commands::{
        COMMAND_SCHEDULER, CollectorType, CommandContext, CommandFeedback,
        CommandType,
    },
    settings::SETTINGS,
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
        if statistics.rolls_remaining == 0 {
            sleep(wait_duration).await;
        }

        for _ in 0..statistics.rolls_remaining {
            let (tx, rx): (
                oneshot::Sender<Option<CommandFeedback>>,
                oneshot::Receiver<Option<CommandFeedback>>,
            ) = oneshot::channel();
            let collector = COMMAND_SCHEDULER
                .default_message_collector(&shard, channel_id);
            COMMAND_SCHEDULER
                .sender()
                .send(CommandContext {
                    command_type: CommandType::Roll(
                        SETTINGS.roll_command.as_str().into(),
                    ),
                    collector: CollectorType::Msg(collector),
                    http: http.clone(),
                    target_channel: channel_id,
                    result_tx: tx,
                })
                .unwrap();

            let a = rx.await.unwrap();
            if let Some(CommandFeedback::Msg(_)) = a {
                statistics.rolls_remaining -= 1;
            }
        }
    }
}
