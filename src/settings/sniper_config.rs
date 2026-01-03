use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    pub id: u64,
}

fn default_roll_after_claim() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct SniperSettings {
    pub guild_id: u64,
    pub instances: Vec<InstanceConfig>,
    pub roll_command: String,
    pub capture_threshold: u32,
    #[serde(default = "default_roll_after_claim")]
    pub roll_after_claim: bool,
    pub rt_capture_threshold: u32,
}
