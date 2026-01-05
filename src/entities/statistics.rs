use chrono::DateTime;
use chrono_tz::Tz;

#[derive(Debug, Clone, Copy)]
pub struct Statistics {
    pub claim_time: DateTime<Tz>,
    pub rolls_remaining: u8,
    pub next_rolls: DateTime<Tz>,
    pub next_daily: DateTime<Tz>,
    pub next_kakera_react: DateTime<Tz>,
    pub kakera_power: u8,
    pub kakera_cost: u8,
    pub kakera_cost_half: u8,
    pub kakera_stock: u32,
    pub next_rt: Option<DateTime<Tz>>,
    pub next_dk: DateTime<Tz>,
    pub rolls_reset_stock: u16,
    pub can_claim: bool,
}
