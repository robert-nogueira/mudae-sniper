pub struct Settings {
    pub token: String,
    pub guild_id: u64,
    pub channels_ids: Vec<u64>,
    pub client_id: u64,
}

impl Settings {
    pub fn load() -> Settings {
        dotenv::dotenv().ok();
        Settings {
            token: dotenv::var("CLIENT_TOKEN")
                .expect("Missing environment variable 'CLIENT_TOKEN'"),
            guild_id: dotenv::var("GUILD_ID")
                .expect("Missing environment variable 'GUILD_ID'")
                .parse()
                .expect("Invalid environment variable 'GUILD_ID'"),
            channels_ids: dotenv::var("CHANNELS")
                .expect("Missing environment variable 'CHANNELS'")
                .split_terminator(",")
                .filter_map(|x| x.parse::<u64>().ok())
                .collect(),
            client_id: dotenv::var("CLIENT_ID")
                .expect("Missing environment variable 'CLIENT_ID'")
                .parse()
                .expect("Invalid environment variable 'CLIENT_ID'"),
        }
    }
}
