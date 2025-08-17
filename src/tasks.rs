use std::{sync::Arc, time::Duration as TimeDuration};

use chrono::{DateTime, Duration};
use chrono_tz::Tz;
use serenity_self::{
    all::{
        Message, MessageCollector, Reaction, ReactionCollector, ReactionType,
        ShardMessenger,
    },
    futures::StreamExt,
};
use tokio::{sync::Mutex, time::sleep};

use crate::{
    snipers::Sniper,
    utils::{REGEX_GET_NUMBERS, get_local_time},
};

pub async fn daily_claimer(
    sniper_mutex: Arc<Mutex<Sniper>>,
    shard: ShardMessenger,
) {
    let mut next_daily: DateTime<Tz>;
    let mut running: bool;
    {
        let sniper = sniper_mutex.lock().await;
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
            (sniper.channel_id, sniper.http.clone())
        };

        channel_id
            .say(&http, "$daily")
            .await
            .expect("fail on send &daily");

        let mut collector = ReactionCollector::new(&shard)
            .channel_id(channel_id)
            .author_id(432610292342587392.into())
            .timeout(TimeDuration::from_secs(30))
            .filter(move |r: &Reaction| match &r.emoji {
                ReactionType::Unicode(unicode) => unicode == "✅",
                _ => false,
            })
            .stream();

        match collector.next().await {
            Some(reaction) => {
                let mut sniper = sniper_mutex.lock().await;
                sniper.statistics.next_daily =
                    get_local_time() + Duration::hours(20);
                next_daily = sniper.statistics.next_daily;
                running = sniper.running;
            }
            None => {
                continue;
            }
        }
    }
}

pub async fn daily_kakera_claimer(
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

        channel_id
            .say(&http, "$dk")
            .await
            .expect("fail on send &daily");
        let mut collector = MessageCollector::new(&shard)
            .channel_id(channel_id)
            .author_id(432610292342587392.into())
            .timeout(TimeDuration::from_secs(30))
            .filter(move |m: &Message| REGEX_GET_NUMBERS.is_match(&m.content))
            .stream();

        match collector.next().await {
            Some(msg) => {
                let stock_value = REGEX_GET_NUMBERS
                    .find_iter(&msg.content)
                    .next()
                    .and_then(|m| parse_num(m.as_str()));

                let mut sniper = sniper_mutex.lock().await;
                sniper.statistics.kakera_stock =
                    stock_value.unwrap_or(sniper.statistics.kakera_stock);
                sniper.statistics.next_dk =
                    get_local_time() + Duration::hours(20);
                next_dk = sniper.statistics.next_dk;
                running = sniper.running;
            }
            None => {
                continue;
            }
        }
    }
}
