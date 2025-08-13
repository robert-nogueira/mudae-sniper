pub mod errors;
pub mod sniper;
pub mod statistics;

use std::sync::{Arc, LazyLock};

use dashmap::DashMap;

pub use errors::ExtractStatisticsError;
pub use sniper::Sniper;
pub use statistics::Statistics;
use tokio::sync::Mutex;

pub static SNIPERS: LazyLock<DashMap<u64, Arc<Mutex<Sniper>>>> = LazyLock::new(DashMap::new);
