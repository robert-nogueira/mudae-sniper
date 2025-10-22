use std::{collections::HashMap, sync::LazyLock};

static BASE_BADGES_PRICES: LazyLock<HashMap<BadgeType, u16>> =
    LazyLock::new(|| {
        HashMap::from([
            (BadgeType::Bronze, 1000),
            (BadgeType::Silver, 2000),
            (BadgeType::Gold, 3000),
            (BadgeType::Sapphire, 5000),
            (BadgeType::Ruby, 7000),
            (BadgeType::Emerald, 9000),
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
