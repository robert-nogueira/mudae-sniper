pub mod errors;
pub mod sniper;

use dashmap::DashMap;
use std::sync::{Arc, LazyLock};

pub use sniper::Sniper;

use serenity_self::all::ChannelId;
use tokio::sync::RwLock;

pub static SNIPERS: LazyLock<DashMap<ChannelId, Arc<RwLock<Sniper>>>> =
    LazyLock::new(DashMap::new);
