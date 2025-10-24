use std::{collections::HashMap, sync::LazyLock};

use crate::settings::SETTINGS;

static BASE_BADGES_PRICES: LazyLock<HashMap<BadgeType, u16>> =
    LazyLock::new(|| {
        HashMap::from([
            (BadgeType::Bronze, SETTINGS.mudae.base_bronze_value),
            (BadgeType::Silver, SETTINGS.mudae.base_bronze_value),
            (BadgeType::Gold, SETTINGS.mudae.base_bronze_value),
            (BadgeType::Sapphire, SETTINGS.mudae.base_bronze_value),
            (BadgeType::Ruby, SETTINGS.mudae.base_bronze_value),
            (BadgeType::Emerald, SETTINGS.mudae.base_bronze_value),
        ])
    });

#[derive(Hash, Eq, PartialEq)]
pub enum BadgeLevel {
    One,
    Two,
    Three,
    Four,
}

#[derive(Hash, Eq, PartialEq)]
pub enum BadgeType {
    Bronze,
    Silver,
    Gold,
    Sapphire,
    Ruby,
    Emerald,
}

pub struct Badge {
    pub level: BadgeLevel,
    pub badge_type: BadgeType,
}

impl BadgeType {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "bronze" => Some(Self::Bronze),
            "silver" => Some(Self::Silver),
            "gold" => Some(Self::Gold),
            "sapphire" => Some(Self::Sapphire),
            "ruby" => Some(Self::Ruby),
            "emerald" => Some(Self::Emerald),
            _ => None,
        }
    }
}

impl BadgeLevel {
    pub fn next_level(&self) -> Option<BadgeLevel> {
        match self {
            BadgeLevel::One => Some(BadgeLevel::Two),
            BadgeLevel::Two => Some(BadgeLevel::Three),
            BadgeLevel::Three => Some(BadgeLevel::Four),
            BadgeLevel::Four => None,
        }
    }

    pub fn previous_level(&self) -> Option<BadgeLevel> {
        match self {
            BadgeLevel::One => None,
            BadgeLevel::Two => Some(BadgeLevel::One),
            BadgeLevel::Three => Some(BadgeLevel::Two),
            BadgeLevel::Four => Some(BadgeLevel::Three),
        }
    }

    pub fn from_number(level: u8) -> Option<Self> {
        match level {
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::Three),
            4 => Some(Self::Four),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            BadgeLevel::One => 1,
            BadgeLevel::Two => 2,
            BadgeLevel::Three => 3,
            BadgeLevel::Four => 4,
        }
    }
}

impl Badge {
    fn value(&self) -> u16 {
        BASE_BADGES_PRICES[&self.badge_type] * self.level.as_u8() as u16
    }

    fn upgrade_cost(&self) -> Option<u16> {
        let next_level = self.level.next_level()?;
        Some(BASE_BADGES_PRICES[&self.badge_type] * next_level.as_u8() as u16)
    }
}
