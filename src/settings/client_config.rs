use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClientSettings {
    pub token: String,
    pub client_id: u64,
}
