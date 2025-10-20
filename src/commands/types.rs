use core::fmt;
use serenity_self::all::{Message, Reaction};

use crate::settings::SETTINGS;

pub enum RollType {
    Waifu,
    Wa,
    Wg,
    Husband,
    Ha,
    Hg,
}

pub enum CommandType {
    Daily,
    DailyKakera,
    Roll(RollType),
    Tu,
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = &SETTINGS.prefix;
        match &self {
            CommandType::Daily => write!(f, "{}daily", prefix),
            CommandType::DailyKakera => write!(f, "{}dk", prefix),
            CommandType::Roll(roll_type) => match roll_type {
                RollType::Waifu => write!(f, "{}w", prefix),
                RollType::Wa => write!(f, "{}wa", prefix),
                RollType::Wg => write!(f, "{}wg", prefix),
                RollType::Husband => write!(f, "{}h", prefix),
                RollType::Ha => write!(f, "{}ha", prefix),
                RollType::Hg => write!(f, "{}hg", prefix),
            },
            CommandType::Tu => write!(f, "{}tu", prefix),
        }
    }
}

pub enum FeedbackType {
    Message,
    Reaction,
}

#[derive(Clone)]
pub enum CommandFeedback {
    Msg(Message),
    React(Reaction),
}
