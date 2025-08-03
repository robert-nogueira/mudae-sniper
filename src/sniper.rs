use serenity_self::all::{ActionRowComponent, ButtonKind, Message};

use crate::models::kakera::Kakera;

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

    pub async fn snipe_kakera(&self, message: &Message) -> Option<Kakera> {
        if message.components.is_empty() {
            return None;
        };

        if message.author.id != 432610292342587392
            || message.channel_id != self.channel_id
            || message.embeds.is_empty()
            || message.components.is_empty()
        {
            return None;
        }

        let kakera_button: &ActionRowComponent = &message.components[0].components[0];

        if let ActionRowComponent::Button(button) = kakera_button {
            if let ButtonKind::NonLink { custom_id, .. } = &button.data {
                self.click_button(custom_id);
            }
        };

        Some(Kakera::Blue(10))
    }
}
