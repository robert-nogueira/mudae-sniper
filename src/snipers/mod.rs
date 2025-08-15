pub mod sniper;
pub mod statistics;

use std::sync::{Arc, LazyLock};

use dashmap::DashMap;

use serenity_self::all::ChannelId;
pub use sniper::Sniper;
pub use statistics::Statistics;
use tokio::sync::Mutex;

pub static SNIPERS: LazyLock<DashMap<ChannelId, Arc<Mutex<Sniper>>>> =
    LazyLock::new(DashMap::new);
