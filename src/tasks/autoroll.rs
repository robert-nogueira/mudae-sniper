use std::{sync::Arc, time::Duration as TimeDuration};

use log::{debug, info};
use serenity_self::all::{Embed, ShardMessenger};
use tokio::{
    sync::{Mutex, oneshot},
    time::sleep,
};

use crate::{
    commands::{
        COMMAND_SCHEDULER, CollectorType, CommandContext, CommandFeedback,
        CommandType,
    },
    entities::badge::BadgeType,
    settings::SETTINGS,
    snipers::{SNIPERS, Sniper},
    utils::{REGEX_GET_NUMBERS, get_local_time},
};

fn extract_kakera_value(embed: &Embed) -> u32 {
    let desc = embed
        .description
        .clone()
        .expect("no description to extract kakera value");
    let last_line = desc.split("\n").last().expect("invalid card description");
    let value_str = REGEX_GET_NUMBERS
        .find(last_line)
        .expect("kakera value not find in description");
    value_str
        .as_str()
        .parse::<u32>()
        .expect("fail on parse kakera value")
}

pub async fn roll_cards(
    sniper_mutex: Arc<Mutex<Sniper>>,
    shard: ShardMessenger,
) {
    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);
    {
        println!("{}", SNIPERS.len());
    }
    let (instance, http, has_rt) = {
        let sniper = sniper_mutex.lock().await;
        info!(
            target: "mudae_sniper",
            instance:? = sniper.instance_ref().name;
            "📝 task started: auto_roll"
        );
        let has_rt = sniper
            .badges
            .iter()
            .any(|badge| badge.badge_type == BadgeType::Emerald);
        (sniper.instance_copy(), sniper.http.clone(), has_rt)
    };
    loop {
        let mut statistics;
        let mut running;
        {
            let sniper = sniper_mutex.lock().await;
            statistics = sniper.statistics_copy();
            running = sniper.running;
        }

        while !running {
            debug!(
                target: "mudae_sniper",
            instance:? = instance.name;
            "🕙 task auto_roll: instance is stopped, trying task again after {CHECK_INTERVAL:?}"
                );
            sleep(CHECK_INTERVAL).await;
            let sniper = sniper_mutex.lock().await;
            statistics = sniper.statistics_copy();
            running = sniper.running;
        }

        let wait_duration = (statistics.next_rolls - get_local_time())
            .to_std()
            .unwrap_or(TimeDuration::ZERO);
        if statistics.rolls_remaining == 0 {
            sleep(wait_duration).await;
        }

        let mut captured: bool = false;
        for _ in 0..statistics.rolls_remaining {
            let (tx, rx): (
                oneshot::Sender<Option<CommandFeedback>>,
                oneshot::Receiver<Option<CommandFeedback>>,
            ) = oneshot::channel();
            let collector = COMMAND_SCHEDULER
                .default_message_collector(&shard, instance.channel_id);
            COMMAND_SCHEDULER
                .sender()
                .send(CommandContext {
                    command_type: CommandType::Roll(
                        SETTINGS.sniper.roll_command.as_str().into(),
                    ),
                    collector: CollectorType::Msg(collector),
                    http: http.clone(),
                    target_instance: instance.clone(),
                    result_tx: tx,
                })
                .unwrap();
            if let Some(CommandFeedback::Msg(msg)) = rx.await.unwrap() {
                statistics.rolls_remaining -= 1;
                let card = msg.embeds[0].clone();
                let kakera_value = extract_kakera_value(&card);
                if captured && has_rt {
                    if kakera_value >= SETTINGS.sniper.rt_capture_threshold {
                        let mut sniper = sniper_mutex.lock().await;
                        if sniper.capture_card(&msg).await.is_ok()
                            && !SETTINGS.sniper.roll_after_claim
                        {
                            break;
                        }
                    }
                } else if kakera_value >= SETTINGS.sniper.capture_threshold {
                    let mut sniper = sniper_mutex.lock().await;
                    if sniper.capture_card(&msg).await.is_ok() {
                        captured = true;
                        if !SETTINGS.sniper.roll_after_claim {
                            break;
                        }
                    }
                }
            }
        }
        let mut sniper = sniper_mutex.lock().await;
        sniper.update_statistics().await;
    }
}
