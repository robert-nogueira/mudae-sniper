pub mod errors;
pub mod sniper;
pub mod statistics;

use dashmap::DashMap;
use std::sync::{Arc, LazyLock};

pub use sniper::Sniper;
pub use statistics::Statistics;

use serenity_self::all::ChannelId;
use tokio::sync::Mutex;

pub static SNIPERS: LazyLock<DashMap<ChannelId, Arc<Mutex<Sniper>>>> =
    LazyLock::new(DashMap::new);
