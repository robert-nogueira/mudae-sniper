use std::collections::HashMap;

use serenity_self::{
    all::{Context, EventHandler, Message},
    async_trait,
};

use crate::sniper::Sniper;

pub struct Handler {
    pub snipers: HashMap<u64, Sniper>,
}

// if let Some(guild_id) = msg.guild_id {
//     let guild = if let Some(guild) = guild_id.to_guild_cached(&ctx.cache) {
// 	guild
//     } else {
// 	match guild_id.to_partial_guild(&ctx.http).await {
// 	    Ok(partial) => partial.into(),
// 	    Err(_) => return
// 	}
//     };
// };
// println!("{:?}", msg.guild_id);

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if let Some(kakera) = self.snipers[&msg.channel_id.into()]
            .snipe_kakera(&msg)
            .await
        {
            println!("Catch: {kakera}")
        }
    }
}
