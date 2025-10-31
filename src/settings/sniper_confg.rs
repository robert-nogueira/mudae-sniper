use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChannelConfig {
    pub name: String,
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct SniperSettings {
    pub guild_id: u64,
    pub channels: Vec<ChannelConfig>,
    pub roll_command: String,
    pub capture_threshold: u32,
    pub roll_after_claim: bool,
    pub rt_capture_threshold: u32,
}
