use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::kakera::Kakera;
use crate::settings::SETTINGS;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde_json::json;
use serenity_self::all::{
    ActionRowComponent, ButtonKind, ChannelId, GuildId, Http, Message,
};

use super::Statistics;

macro_rules! some_or_continue {
    ($expr:expr) => {
        match $expr {
            Some(v) => v,
            None => continue,
        }
    };
}

pub struct Sniper {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub running: bool,
    pub http: Arc<Http>,
    pub statistics: Statistics,
}

impl Sniper {
    pub fn new(
        channel_id: ChannelId,
        guild_id: GuildId,
        http: Arc<Http>,
        statistics: Statistics,
    ) -> Sniper {
        Sniper {
            channel_id,
            guild_id,
            running: true,
            http,
            statistics,
        }
    }

    async fn click_button(&self, custom_id: &str, message_id: u64) {
        let url = "https://discord.com/api/v10/interactions";
        let session_id = SystemTime::now() // fake session_id
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let body = json!({
            "type" : 3,
            "guild_id": self.guild_id.to_string(),
            "channel_id": self.channel_id.to_string(),
            "message_id": message_id.to_string(),
            "application_id": "432610292342587392",
            "session_id": session_id,
            "data": {"component_type": 2, "custom_id": custom_id}
        });
        Client::new()
            .post(url)
            .header(AUTHORIZATION, SETTINGS.token.to_string())
            .json(&body)
            .send()
            .await
            .expect("HTTP Error");
    }

    pub async fn snipe_kakeras(&mut self, message: &Message) -> Vec<Kakera> {
        let mut kakeras_sniped: Vec<Kakera> = vec![];

        if message.author.id != 432610292342587392
            || message.channel_id != self.channel_id
            || message.embeds.is_empty()
            || message.embeds[0].description.is_none()
            || message.components.is_empty()
        {
            return kakeras_sniped;
        }

        if message.components[0].components.is_empty() {
            return kakeras_sniped;
        }

        for component in &message.components[0].components {
            if self.statistics.kakera_power < self.statistics.kakera_cost {
                continue;
            }
            let button = some_or_continue!(match component {
                ActionRowComponent::Button(button) => Some(button),
                _ => None,
            });

            let custom_id = some_or_continue!(match &button.data {
                ButtonKind::NonLink { custom_id, .. } => Some(custom_id),
                _ => None,
            });

            let button_emoji_id = match button.emoji.clone().unwrap() {
                serenity_self::all::ReactionType::Custom { id, .. } => id,
                _ => continue,
            };
            self.click_button(custom_id, message.id.into()).await;

            let desc =
                some_or_continue!(message.embeds[0].description.clone());
            let value = some_or_continue!(desc.split("\n").last());
            let value: u16 = some_or_continue!(value.parse().ok());
            let kakera = some_or_continue!(Kakera::from_emoji_id(
                button_emoji_id.into(),
                value
            ));
            self.statistics.kakera_power -= self.statistics.kakera_cost;
            self.statistics.kakera_stock += value as u32;
            kakeras_sniped.push(kakera);
        }
        kakeras_sniped
    }
}
