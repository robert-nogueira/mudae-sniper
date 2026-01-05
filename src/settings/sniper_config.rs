use serde::Deserialize;

fn default_roll_after_claim() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    pub id: u64,
    #[serde(default = "default_roll_after_claim")]
    pub roll_after_claim: bool,
}

#[derive(Debug, Deserialize)]
pub struct SniperSettings {
    pub guild_id: u64,
    pub instances: Vec<InstanceConfig>,
    pub roll_command: String,
    pub capture_threshold: u32,
    // pub rt_capture_threshold: u32,
}
