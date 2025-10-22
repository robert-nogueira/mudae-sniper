use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SniperSettings {
    pub guild_id: u64,
    pub channels_ids: Vec<u64>,
    pub roll_command: String,
    pub mudae_prefix: String,
    pub capture_threshold: u32,
    pub roll_after_claim: bool,
}
