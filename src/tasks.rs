use std::{sync::Arc, time::Duration as TimeDuration};

use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use serenity_self::{
    all::{
        Context, Message, MessageCollector, Reaction, ReactionCollector,
        ReactionType, ShardMessenger,
    },
    futures::StreamExt,
};
use tokio::{sync::Mutex, time::sleep};

use crate::{
    snipers::{Sniper, sniper},
    utils::REGEX_GET_NUMBERS,
};

// pub async fn roll(sniper: Arc<Mutex<Sniper>>) {
//     loop {

//     }
// }

pub async fn daily_claimer(
    sniper_mutex: Arc<Mutex<Sniper>>,
    shard: &ShardMessenger,
) {
    let mut next_daily: DateTime<Utc>;
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
        sleep(
            (next_daily - Utc::now())
                .to_std()
                .unwrap()
                .max(TimeDuration::ZERO),
        )
        .await;
        let (channel_id, http) = {
            let sniper = sniper_mutex.lock().await;
            (sniper.channel_id, sniper.http.clone())
        };
        channel_id
            .say(&http, "$daily")
            .await
            .expect("fail on send &daily");
        let mut collector = ReactionCollector::new(shard)
            .channel_id(channel_id)
            .author_id(432610292342587392.into())
            .timeout(TimeDuration::from_secs(30))
            .filter(move |r: &Reaction| match &r.emoji {
                ReactionType::Unicode(unicode) => unicode == "✅",
                _ => false,
            })
            .stream();
        match collector.next().await {
            Some(_) => {
                let mut sniper = sniper_mutex.lock().await;
                sniper.statistics.next_daily =
                    Utc::now() + Duration::hours(20);
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
    shard: &ShardMessenger,
) {
    fn parse_num(text: &str) -> Option<u32> {
        text.replace(",", "").replace(".", "").parse().ok()
    }

    let mut next_daily: DateTime<Utc>;
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
        sleep(
            (next_daily - Utc::now())
                .to_std()
                .unwrap()
                .max(TimeDuration::ZERO),
        )
        .await;
        let (channel_id, http) = {
            let sniper = sniper_mutex.lock().await;
            (sniper.channel_id, sniper.http.clone())
        };
        channel_id
            .say(&http, "$daily")
            .await
            .expect("fail on send &daily");
        let mut collector = MessageCollector::new(shard)
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
                sniper.statistics.next_daily =
                    Utc::now() + Duration::hours(20);
                next_daily = sniper.statistics.next_daily;
                running = sniper.running;
            }
            None => {
                continue;
            }
        }
    }
}
