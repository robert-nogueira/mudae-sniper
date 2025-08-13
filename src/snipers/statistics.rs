use chrono::{DateTime, Utc};

pub struct Statistics {
    pub claim_time: DateTime<Utc>,
    pub rolls_remaining: u8,
    pub next_rolls: DateTime<Utc>,
    pub next_daily: DateTime<Utc>,
    pub next_kakera_react: DateTime<Utc>,
    pub kakera_power: u8,
    pub kakera_cost: u8,
    pub kakera_cost_half: u8,
    pub kakera_stock: u32,
    pub next_rt: Option<DateTime<Utc>>,
    pub next_dk: DateTime<Utc>,
    pub rolls_reset_stock: u16,
}
