use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::kakera::Kakera;
use crate::settings::SETTINGS;
use crate::utils::extract_statistics;
use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
use serde_json::json;
use serenity_self::all::{ActionRowComponent, ButtonKind, ChannelId, GuildId, Http, Message};

use super::{ExtractStatisticsError, Statistics};

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
    pub statistics: Option<Statistics>,
}

impl Sniper {
    pub fn new(channel_id: ChannelId, guild_id: GuildId, http: Arc<Http>) -> Sniper {
        Sniper {
            channel_id,
            guild_id,
            running: true,
            http,
            statistics: None,
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

    pub async fn snipe_kakeras(&self, message: &Message) -> Vec<Kakera> {
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

            let desc = some_or_continue!(message.embeds[0].description.clone());
            let value = some_or_continue!(desc.split("\n").last());
            let value: u16 = some_or_continue!(value.parse().ok());
            let kakera = some_or_continue!(Kakera::from_emoji_id(button_emoji_id.into(), value));

            kakeras_sniped.push(kakera);
        }
        kakeras_sniped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status_ptbr() {
        let text = "**allma_**, Calma aí, falta um tempo antes que você possa se casar novamente **1h 09** min.
Você tem **17** rolls restantes.
A próxima reinicialização é em **39** min.
Próximo reset do $daily em **19h 54** min.
Você __pode__ pegar kakera agora!
Power: **100%**
Cada reação de kakera consume 70% de seu reaction power.
Seus Personagens com 10+ chaves consome metade do power (35%)
Stock: **9.040**<:kakera:469835869059153940>
$rt está pronto!
$dk está pronto!
Você tem **37** rolls reset no estoque";
        let status = extract_statistics(text);
        assert!(status.is_some());
    }

    #[test]
    fn test_get_status_en() {
        let text = "**allma_**, you __can__ claim right now! The next claim reset is in **25** min.
You have **10** rolls left. Next rolls reset in **25** min.
Next $daily reset in **11h 32** min.
You __can__ react to kakera right now!
Power: **100%**
Each kakera reaction consumes 100% of your reaction power.
Your characters with 10+ keys consume half the power (50%)
Stock: **0**<:kakera:469835869059153940>
$dk is ready!
You have **38** rolls reset in stock.";
        let status = extract_statistics(text);
        assert!(status.is_some());
    }

    #[test]
    fn test_get_status_fr() {
        let text = "**allma_**, vous __pouvez__ vous marier dès maintenant ! Le prochain reset est dans **24** min.
Vous avez **10** rolls restants.
Prochain rolls reset dans **24** min.
Prochain $daily reset dans **11h 31** min.
Vous __pouvez__ réagir aux kakera dès maintenant !
Power: **100%**
Chaque réaction à un kakera consomme 100% de votre pouvoir de réaction.
Vos personnages possédant 10+ keys consomment moitié moins de pouvoir (50%)
Stock: **0**<:kakera:469835869059153940>
$dk est prêt !
Vous avez **38** rolls reset en stock.";
        let status = extract_statistics(text);
        assert!(status.is_some());
    }

    #[test]
    fn test_get_status_es() {
        let text =
            "**allma_**, __puedes__ reclamar ahora mismo. El siguiente reclamo será en **24** min.
Tienes **10** rolls restantes.
El siguiente reinicio será en **24** min.
Siguiente reinicio de $daily en **11h 31** min.
¡__Puedes__ reaccionar a kakera en este momento!
Poder: **100%**
Cada reacción de kakera consume 100% de su poder de reacción.
Tus personajes con 10+ llaves, consumen la mitad del poder (50%)
Capital: **256,838**<:kakera:469835869059153940>
¡$dk está listo!
Tienes **38** reinicios de rolls en el inventario.";
        let status = extract_statistics(text);
        assert!(status.is_some());
    }
}
