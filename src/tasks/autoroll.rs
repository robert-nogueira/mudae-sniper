use std::{sync::Arc, time::Duration as TimeDuration};

use log::{debug, info};
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
    entities::badge::BadgeType,
    settings::SETTINGS,
    snipers::Sniper,
    utils::{fmt_duration_from_now, get_local_time},
};

pub async fn roll_cards(
    sniper_mutex: Arc<Mutex<Sniper>>,
    shard: ShardMessenger,
) {
    const CHECK_INTERVAL: TimeDuration = TimeDuration::from_secs(60);
    let (instance, http, _has_rt) = {
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
                instance:? = &instance.name;
                "🕙 task auto_roll: instance is stopped, trying task again after {CHECK_INTERVAL:?}"
            );
            sleep(CHECK_INTERVAL).await;
            let sniper = sniper_mutex.lock().await;
            statistics = sniper.statistics_copy();
            running = sniper.running;
        }

        let should_wait = if !instance.roll_after_claim {
            !statistics.can_claim || statistics.rolls_remaining == 0
        } else {
            statistics.rolls_remaining == 0
        };

        if should_wait {
            let now = get_local_time();

            let target_time =
                if !instance.roll_after_claim && !statistics.can_claim {
                    statistics.claim_time
                } else {
                    statistics.next_rolls
                };

            let wait_duration =
                (target_time - now).to_std().unwrap_or(TimeDuration::ZERO);

            debug!(
            target: "mudae_sniper",
            instance:? = &instance.name;
            "⏳ waiting {} until {} (can_claim={}, rolls_remaining={}, strategy={})",
            fmt_duration_from_now(target_time, now),
            if !instance.roll_after_claim && !statistics.can_claim { "claim_time" } else { "next_rolls" },
            statistics.can_claim,
            statistics.rolls_remaining,
            if instance.roll_after_claim { "roll_after_claim" } else { "stop_after_claim" }
                );

            sleep(wait_duration).await;
            let sniper = sniper_mutex.lock().await;
            statistics = sniper.statistics_copy();
        }

        debug!(
            target: "mudae_sniper",
            instance:? = &instance.name;
            "🎲 rolling {} cards",
            statistics.rolls_remaining
        );

        for roll_num in 0..statistics.rolls_remaining {
            let (tx, _): (
                oneshot::Sender<Option<CommandFeedback>>,
                oneshot::Receiver<Option<CommandFeedback>>,
            ) = oneshot::channel();
            let collector = COMMAND_SCHEDULER
                .default_message_collector(&shard, instance.channel_id)
                .filter(|msg| !msg.embeds.is_empty());

            debug!(
                target: "mudae_sniper",
                instance:? = &instance.name;
                "🎲 sending roll command {}/{}",
                roll_num + 1,
                statistics.rolls_remaining
            );

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
        }

        let now = get_local_time();
        let wait_duration = (statistics.next_rolls - now)
            .to_std()
            .unwrap_or(TimeDuration::ZERO);

        debug!(
            target: "mudae_sniper",
            instance:? = &instance.name;
            "⏳ waiting {} until next_rolls",
            fmt_duration_from_now(statistics.next_rolls, now)
        );
        {
            let mut sniper = sniper_mutex.lock().await;
            sniper.update_statistics().await.expect(
                "Failed on update statistics. Check the logs for details",
            );
        }
        sleep(wait_duration).await;
    }
}
