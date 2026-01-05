use serenity_self::all::ChannelId;

#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub channel_id: ChannelId,
    pub roll_after_claim: bool,
}

impl Instance {
    pub fn id_as_u64(&self) -> u64 {
        self.channel_id.into()
    }
}
