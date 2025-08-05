use std::{collections::HashMap, sync::Arc};

use serenity_self::all::{Context, EventHandler, Message, async_trait};

use crate::{settings::Settings, sniper::Sniper};

pub struct Handler {
    pub snipers: HashMap<u64, Sniper>,
    pub settings: Arc<Settings>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(sniper) = self.snipers.get(&msg.channel_id.into()) {
            if !sniper.running {
                return;
            }
            let my_id = self.settings.client_id;
            if msg.author.id == my_id {
                return;
            }
            if let Some(kakera) = sniper.snipe_kakera(&ctx, &msg).await {
                println!("Catch: {kakera}")
            }
        }
    }
}
