use std::time::Duration;

use crate::models::kakera::Kakera;
use serenity_self::Error;
use serenity_self::all::{
    ActionRowComponent, ButtonKind, ChannelId, Context, CreateMessage, Message,
};
use serenity_self::collector::MessageCollector;
use serenity_self::futures::StreamExt;

pub struct Sniper {
    pub guild_id: u64,
    pub channel_id: u64,
}

impl Sniper {
    pub async fn new(channel_id: u64, guild_id: u64) -> Sniper {
        Sniper {
            channel_id,
            guild_id,
        }
    }

    fn click_button(&self, custom_id: &str) {
        println!("{}{}", self.guild_id, custom_id);
    }

    async fn check_ku(&self, ctx: &Context) -> Result<Option<u8>, Error> {
        let msg = CreateMessage::new().content("$ku");
        let channel: ChannelId = self.channel_id.into();
        channel.send_message(&ctx.http, msg).await?;
        let mut collector = MessageCollector::new(&ctx.shard)
            .author_id(432610292342587392.into())
            .channel_id(channel)
            .timeout(Duration::from_secs(10))
            .stream();
        if let Some(msg) = collector.next().await {
            println!("msg: {}", msg.content);
            return Ok(Some(1));
        }
        Ok(None)
    }

    pub async fn snipe_kakera(&self, ctx: &Context, message: &Message) -> Option<Kakera> {
        self.check_ku(ctx).await.ok()??;
        if message.components.is_empty() {
            return None;
        };

        if message.author.id != 432610292342587392
            || message.channel_id != self.channel_id
            || message.embeds.is_empty()
            || message.embeds[0].description.is_none()
            || message.components.is_empty()
        {
            return None;
        }

        let button = match &message.components[0].components[0] {
            ActionRowComponent::Button(button) => Some(button),
            _ => None,
        }?;

        let custom_id = match &button.data {
            ButtonKind::NonLink { custom_id, .. } => Some(custom_id),
            _ => None,
        }?;

        self.click_button(custom_id);

        let desc = message.embeds[0].description.clone()?;
        let (value, name) = desc.split_once(":")?;
        let value: u16 = value.parse().ok()?; // i love shadowing
        match name {
            "kakeraL" => Some(Kakera::Light(value)),
            _ => None,
        }
    }
}
