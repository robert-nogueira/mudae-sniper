use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::commands::{
    COMMAND_SCHEDULER, CollectorType, CommandContext, CommandFeedback,
    CommandType,
};
use crate::entities::badge::Badge;
use crate::entities::instance::Instance;
use crate::entities::kakera::Kakera;
use crate::entities::statistics::Statistics;
use crate::settings::SETTINGS;
use crate::utils::extract_statistics;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde_json::json;
use serenity_self::all::{
    ActionRowComponent, ButtonKind, ChannelId, GuildId, Http, Message,
    MessageId, ShardMessenger,
};
use tokio::sync::oneshot;

use super::errors::CaptureError;

macro_rules! some_or_continue {
    ($expr:expr) => {
        match $expr {
            Some(v) => v,
            None => continue,
        }
    };
}

pub struct Sniper {
    instance: Instance,
    statistics: Statistics,
    pub guild_id: GuildId,
    pub running: bool,
    pub http: Arc<Http>,
    pub shard: ShardMessenger,
    pub badges: Vec<Badge>,
}

impl Sniper {
    pub fn new(
        channel_id: ChannelId,
        guild_id: GuildId,
        http: Arc<Http>,
        shard: ShardMessenger,
        statistics: Statistics,
        badges: Vec<Badge>,
        instance_name: String,
    ) -> Sniper {
        Sniper {
            guild_id,
            running: true,
            http,
            shard,
            statistics,
            badges,
            instance: Instance {
                channel_id,
                name: instance_name,
            },
        }
    }

    pub fn instance_copy(&self) -> Instance {
        self.instance.clone()
    }

    pub fn instance_ref(&self) -> &Instance {
        &self.instance
    }

    pub fn statistics_copy(&self) -> Statistics {
        self.statistics
    }

    pub fn statistics_ref(&self) -> &Statistics {
        &self.statistics
    }

    async fn click_button(&self, custom_id: &str, message_id: MessageId) {
        let url = "https://discord.com/api/v10/interactions";
        let session_id = SystemTime::now() // fake session_id
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let body = json!({
            "type" : 3,
            "guild_id": self.guild_id.to_string(),
            "channel_id": self.instance.channel_id.to_string(),
            "message_id": message_id.to_string(),
            "application_id": "432610292342587392",
            "session_id": session_id,
            "data": {"component_type": 2, "custom_id": custom_id}
        });
        Client::new()
            .post(url)
            .header(AUTHORIZATION, SETTINGS.client.token.to_string())
            .json(&body)
            .send()
            .await
            .expect("HTTP Error");
    }

    pub async fn update_statistics(&mut self) {
        let (tx, rx): (
            oneshot::Sender<Option<CommandFeedback>>,
            oneshot::Receiver<Option<CommandFeedback>>,
        ) = oneshot::channel();
        let collector = COMMAND_SCHEDULER
            .default_message_collector(&self.shard, self.instance.channel_id);
        COMMAND_SCHEDULER
            .sender()
            .send(CommandContext {
                command_type: CommandType::Tu,
                collector: CollectorType::Msg(collector),
                http: self.http.clone(),
                target_instance: self.instance.clone(),
                result_tx: tx,
            })
            .unwrap();
        if let Some(CommandFeedback::Msg(msg)) = rx.await.unwrap() {
            self.statistics = extract_statistics(&msg.content)
                .expect("error on extract statistics");
        }
    }

    pub async fn capture_card(
        &mut self,
        message: &Message,
    ) -> Result<(), CaptureError> {
        let button = message
            .components
            .first()
            .and_then(|row| row.components.first())
            .ok_or(CaptureError::MissingComponent)?;
        let custom_id: String = match button {
            ActionRowComponent::Button(button) => match &button.data {
                ButtonKind::NonLink { custom_id, .. } => custom_id.to_string(),
                _ => {
                    return Err(CaptureError::InvalidButton(
                        button.data.clone(),
                    ));
                }
            },
            _ => return Err(CaptureError::NotAButton(button.clone())),
        };
        self.click_button(&custom_id, message.id).await;
        self.update_statistics().await;
        Ok(())
    }

    pub async fn snipe_kakeras(&mut self, message: &Message) -> Vec<Kakera> {
        let mut kakeras_sniped: Vec<Kakera> = vec![];

        if message.author.id != 432610292342587392
            || message.channel_id != self.instance.channel_id
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
            self.click_button(custom_id, message.id).await;

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
