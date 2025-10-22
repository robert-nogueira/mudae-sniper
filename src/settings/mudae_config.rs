use serde::Deserialize;

fn default_prefix() -> String {
    "$".to_string()
}
fn default_base_bronze_value() -> u16 {
    1000
}
fn default_base_silver_value() -> u16 {
    2000
}
fn default_base_gold_value() -> u16 {
    3000
}
fn default_base_sapphire_value() -> u16 {
    5000
}
fn default_base_ruby_value() -> u16 {
    7000
}
fn default_base_emerald_value() -> u16 {
    9000
}

#[derive(Debug, Deserialize)]
pub struct MudaeSettings {
    #[serde(default = "default_prefix")]
    pub prefix: String,
    #[serde(default = "default_base_bronze_value")]
    pub base_bronze_value: u16,
    #[serde(default = "default_base_silver_value")]
    pub base_silver_value: u16,
    #[serde(default = "default_base_gold_value")]
    pub base_gold_value: u16,
    #[serde(default = "default_base_sapphire_value")]
    pub base_sapphire_value: u16,
    #[serde(default = "default_base_ruby_value")]
    pub base_ruby_value: u16,
    #[serde(default = "default_base_emerald_value")]
    pub base_emerald_value: u16,
}
