use std::collections::HashMap;

use serenity_self::{
    all::{Context, EventHandler, Message},
    async_trait,
};

use crate::sniper::Sniper;

pub struct Handler {
    pub snipers: HashMap<u64, Sniper>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(sniper) = self.snipers.get(&msg.channel_id.into()) {
            if let Some(kakera) = sniper.snipe_kakera(&ctx, &msg).await {
                println!("Catch: {kakera}")
            }
        }
    }
}
