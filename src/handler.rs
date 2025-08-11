use std::collections::HashMap;

use serenity_self::all::{Context, EventHandler, Message, async_trait};

use crate::{settings::SETTINGS, sniper::Sniper};

pub struct Handler {
    pub snipers: HashMap<u64, Sniper>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.guild_id.is_none() && msg.content != "!start" {
            // setup();
        }
        if let Some(sniper) = self.snipers.get(&msg.channel_id.into()) {
            if !sniper.running {
                return;
            }

            let my_id = SETTINGS.client_id;
            if msg.author.id == my_id {
                return;
            }
            if let Some(kakera) = sniper.snipe_kakera(&ctx, &msg).await {
                println!("Catch: {kakera}")
            }
        }
    }
}
