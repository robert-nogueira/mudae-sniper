use serenity_self::all::{Context, EventHandler, Message, async_trait};

use crate::{settings::SETTINGS, snipers::SNIPERS};

pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let current_user_id = ctx.cache.current_user().id;
        if let Some(sniper) = SNIPERS.get(&msg.channel_id.into()) {
            let sniper = sniper.lock().await;

            if msg.author.id == current_user_id && msg.content != "!start" {
                // setup()
            }

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
